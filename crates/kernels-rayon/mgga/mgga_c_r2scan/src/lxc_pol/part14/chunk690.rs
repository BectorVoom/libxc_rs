//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 690/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk690(t423: f64, t5249: f64, t170: f64, t1727: f64, t597: f64, t1375: f64, t1859: f64, t1862: f64, t1823: f64, t732: f64, t1818: f64, t712: f64) -> (f64, f64, f64, f64, f64) {
    let t5250 = t5249 * t423;
    let t5251 = t170 * t1727;
    let t5252 = t597 * t5251;
    let t5253 = t5250 * t5252;
    let t5255 = t1859 * t1375;
    let t5256 = t5255 * t1862;
    let t5258 = t732 * t1823;
    let t5260 = t1818 * t712;
    (t5252, t5253, t5256, t5258, t5260)
}
