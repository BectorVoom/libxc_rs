//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2239/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2239(t3666: f64, t8184: f64, t17307: f64, t2138: f64, t17451: f64, t26867: f64, t1285: f64, t97173: f64, t104646: f64, t17735: f64, t1238: f64, t16715: f64, t17502: f64, t17541: f64, t17584: f64, t17635: f64, t17696: f64, t17739: f64, t26880: f64, t29047: f64, t3674: f64, t5279: f64, t57549: f64, t97250: f64) -> (f64, f64) {
    let t104924 = t3666 * t8184;
    let t104927 = t17307 * t2138;
    let t104933 = t26867 * t17451;
    let t104943 = t1285 * t97173;
    let t104946 = t17735 * t104646;
    let t104951 = 0.45732285992607719436e-2_f64 * t104924 * t1238 + 0.85748036236139473944e-3_f64 * t104927 * t3674 - 7.0_f64 / 648.0_f64 * t29047 * t57549 * t16715 - 0.3811023832717309953e-3_f64 * t104933 + 0.28582678745379824648e-3_f64 * t26880 * t17541 + 0.57165357490759649296e-3_f64 * t97250 * t5279 + 0.57165357490759649296e-3_f64 * t26880 * t17502 + 0.28582678745379824648e-3_f64 * t26880 * t17584 + 0.95275595817932748826e-3_f64 * t104943 * t17696 - 0.11433071498151929859e-2_f64 * t104946 * t17739 - 0.57165357490759649296e-3_f64 * t26867 * t17635;
    (t104943, t104951)
}
