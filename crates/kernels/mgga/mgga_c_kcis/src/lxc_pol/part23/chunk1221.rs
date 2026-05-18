//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1221/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1221<F: Float>(t97661: F, t97663: F, t97665: F, t97667: F, t97669: F, t97671: F, t97673: F, t97675: F, t97677: F, t97679: F, t97682: F, t97684: F, t97686: F, t97688: F, t97690: F, t97692: F, t97695: F, t97698: F) -> F {
    let t97900 = -t97661 / F::new(128.0) - t97663 / F::new(48.0) - t97665 / F::new(96.0) + t97667 / F::new(18.0) + t97669 / F::new(54.0) + t97671 / F::new(128.0) + t97673 / F::new(4.0) - t97675 / F::new(288.0) + t97677 / F::new(72.0) - t97679 / F::new(96.0) + t97682 / F::new(8.0) + t97684 / F::new(8.0) - t97686 / F::new(9.0) + t97688 / F::new(4.0) + t97690 / F::new(64.0) + t97692 / F::new(16.0) - t97695 / F::new(16.0) - t97698 / F::new(144.0);
    t97900
}
