//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1059/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1059<F: Float>(t10415: F, t10418: F, t10423: F, t10463: F, t19523: F, t2500: F, t28649: F, t28653: F, t28659: F, t28662: F, t28665: F, t28671: F, t28677: F, t3347: F, t34: F, t445: F, t454: F, t6723: F) -> (F,) {
    let t28684 = 50.0 / 81.0 * t454 * t10415 + 40.0 / 81.0 * t34 * t28649 - 10.0 / 9.0 * t19523 * t28653 - 50.0 / 9.0 * t454 * t10418 - 10.0 / 9.0 * t19523 * t28659 + 10.0 / 3.0 * t6723 * t28662 + 10.0 / 3.0 * t34 * t28665 - 25.0 / 9.0 * t454 * t10423 + 10.0 / 9.0 * t34 * t28671 + 5.0 / 3.0 * t34 * t28677 - 2200.0 / 81.0 * t10463 * t445 + 400.0 / 27.0 * t3347 * t2500;
    (t28684,)
}
