//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1229/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1229(t7049: f64, t786: f64, t867: f64, t2467: f64, t2772: f64, t689: f64, t7014: f64, t25338: f64, t887: f64, t10977: f64, t1949: f64, t231: f64, t25317: f64, t25322: f64, t25325: f64, t25383: f64, t25391: f64, t25395: f64, t25407: f64, t25419: f64, t27357: f64, t2829: f64, t7070: f64, t7071: f64, t7076: f64, t7083: f64, t886: f64, t92884: f64, t92891: f64, t92895: f64, t92901: f64, t92905: f64, t92907: f64, t92917: f64) -> f64 {
    let t92921 = t786 * t7049 * t867;
    let t92922 = t92921 * t2467;
    let t92925 = t689 * t7014 * t2772;
    let t92930 = t689 * t25338 * t887;
    let t92932 = -0.78062653693846795158e1_f64 * t7070 * t25317 * t25325 * t886 - 0.13010442282307799193e1_f64 * t25407 * t7083 + 0.52041769129231196772e1_f64 * t25391 * t27357 * t92884 - 0.38554277296572111609e-1_f64 * t92891 + 0.51405703062096148814e-2_f64 * t92895 - 0.19756347548806534796e1_f64 * t25322 * t2829 - 0.16463622957338778996e-1_f64 * t92901 + 0.14456046980341999104e-2_f64 * t92905 + 0.4336814094102599731e0_f64 * t7070 * t7076 * t92907 * t231 + 0.8673628188205199462e0_f64 * t7070 * t7071 * t1949 * t10977 - 0.52041769129231196772e1_f64 * t92917 * t25395 - 0.58544643236296698113e-1_f64 * t92922 - 0.32927245914677557992e-1_f64 * t92925 - 0.26020884564615598386e1_f64 * t25383 * t25419 + 0.32927245914677557992e-1_f64 * t92930;
    t92932
}
