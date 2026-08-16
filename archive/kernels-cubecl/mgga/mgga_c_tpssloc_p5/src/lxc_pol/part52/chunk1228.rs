//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1228/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1228<F: Float>(t1442: F, t32661: F, t32664: F, t32666: F, t32668: F, t32671: F, t32674: F, t32676: F, t32679: F, t32680: F, t32684: F, t32784: F, t33720: F, t574: F, t8682: F) -> F {
    let t33722 = -t1442 * t8682 + t33720 * t574 - F::cast_from(2.0_f64) * t32661 - F::cast_from(2.0_f64) * t32664 - t32666 + F::cast_from(3.0_f64) * t32668 - F::cast_from(2.0_f64) * t32671 - t32674 - t32676 - t32679 - F::cast_from(2.0_f64) * t32680 + t32684 + t32784;
    t33722
}
