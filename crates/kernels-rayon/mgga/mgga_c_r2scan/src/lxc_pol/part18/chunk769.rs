//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 769/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk769(t7125: f64, t1524: f64, t963: f64, t6887: f64, t970: f64, t2271: f64, t2816: f64, t2747: f64, t468: f64, t1411: f64, t1385: f64, t1561: f64, t983: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7126 = 2.0_f64 * t7125;
    let t7127 = t963 * t1524;
    let t7129 = t6887 * t970;
    let t7132 = 0.4726e1_f64 * t2271 * t2816;
    let t7155 = t2747 * t468;
    let t7156 = 0.11696447245269292414e1_f64 * t7155;
    let t7157 = t963 * t1411;
    let t7159 = t963 * t1385;
    let t7217 = t1561 * t983;
    (t7126, t7127, t7129, t7132, t7156, t7157, t7159, t7217)
}
