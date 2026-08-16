//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1254/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1254(t13180: f64, t225: f64, t1294: f64, t3738: f64, t1204: f64, t1210: f64, t1215: f64, t12666: f64, t12673: f64, t12690: f64, t12696: f64, t1271: f64, t1274: f64, t1295: f64, t13166: f64, t13170: f64, t13174: f64, t13177: f64, t3552: f64, t3556: f64, t3561: f64, t3585: f64, t3729: f64, t3732: f64, t3739: f64, t3791: f64, t460: f64, t495: f64) -> (f64, f64, f64, f64) {
    let t13181 = 1.0_f64 / t13180;
    let t13182 = t225 * t13181;
    let t13183 = t3738 * t1294;
    let t13184 = t13182 * t13183;
    let t13189 = 0.39512695097613069591e1_f64 * t3732 * t3739 - 0.19756347548806534796e1_f64 * t12666 * t1215 - 0.19756347548806534796e1_f64 * t3556 * t3585 + 0.39512695097613069591e1_f64 * t3561 * t3739 - 0.19756347548806534796e1_f64 * t12673 * t1295 - 0.19756347548806534796e1_f64 * t3561 * t3791 + 0.65854491829355115987e0_f64 * t12690 * t495 + 0.19756347548806534796e1_f64 * t3552 * t1271 + 0.39512695097613069591e1_f64 * t1274 * t12696 - 0.65854491829355115987e0_f64 * t1274 * t13166 + 0.65854491829355115987e0_f64 * t460 * t13170 - 0.39512695097613069591e1_f64 * t1210 * t13174 - 0.39512695097613069591e1_f64 * t13177 * t1215 - 0.39512695097613069591e1_f64 * t1274 * t13184 + 0.19756347548806534796e1_f64 * t1204 * t3729;
    (t13182, t13183, t13184, t13189)
}
