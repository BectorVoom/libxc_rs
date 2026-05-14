//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 592/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk592<F: Float>(t2: F, t8275: F, t11175: F, t17: F, t9: F, t3141: F, t8282: F, t959: F, t1775: F, t3151: F, t3146: F, t3131: F, t1555: F, t26: F, t1557: F, t469: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11690 = t8275 * t2;
    let t11717 = t9 * t11175 * t17;
    let t11718 = t11717 * t3141;
    let t11720 = t8282 * t959;
    let t11732 = 4.0 / 3.0 * t1775 * t3151;
    let t11734 = 2.0 / 9.0 * t1775 * t3146;
    let t11745 = 2.0 / 9.0 * t1775 * t3131;
    let t11755 = t26 * t1555;
    let t11756 = t469 * t1557;
    (t11690, t11717, t11718, t11720, t11732, t11734, t11745, t11755, t11756)
}
