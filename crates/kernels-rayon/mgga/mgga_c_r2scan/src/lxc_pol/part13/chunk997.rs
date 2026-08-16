//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 997/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk997(t11702: f64, t3295: f64, t3308: f64, t7629: f64, t2184: f64, t8156: f64, t1592: f64, t8160: f64, t7615: f64, t2196: f64, t10760: f64, t7922: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11703 = t3295 * t11702;
    let t11705 = t3308 * t7629;
    let t11706 = t2184 * t11705;
    let t11708 = t3308 * t8156;
    let t11709 = t1592 * t11708;
    let t11711 = t3308 * t8160;
    let t11712 = t1592 * t11711;
    let t11714 = t3308 * t7615;
    let t11715 = t2196 * t11714;
    let t11717 = t10760 * t7922;
    (t11703, t11705, t11706, t11708, t11709, t11711, t11712, t11714, t11715, t11717)
}
