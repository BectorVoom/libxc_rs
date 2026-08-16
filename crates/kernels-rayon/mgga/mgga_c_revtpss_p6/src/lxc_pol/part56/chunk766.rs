//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 766/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk766(t265: f64, t502: f64, t1259: f64, t72: f64, t3720: f64, t8926: f64, t8928: f64, t8932: f64, t8933: f64, t8938: f64, t8941: f64, t8946: f64, t2155: f64, t1300: f64, t198: f64, t336: f64, t3801: f64, t8542: f64) -> (f64, f64, f64, f64, f64) {
    let t503 = t265 < t502;
    let t8947 = t1259 * t72;
    let t8948 = t8947 * t3720;
    let t8951 = 0.28234466758480466999e-3_f64 * t8926 * t8928 - 0.8673628188205199462e0_f64 * t8932 * t8933 + 0.57119737665102352616e0_f64 * t8938 * t8941 - 0.1859366460452550541e-3_f64 * t8946 * t8948;
    let t8955 = t2155 * t2155;
    let t8960 = piecewise3(t503, t1300 * t198 * t336 * t8951 - t198 * t336 * t3801 * t8955, t8542);
    (t8947, t8948, t8951, t8955, t8960)
}
