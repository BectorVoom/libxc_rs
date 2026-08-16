//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1183/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1183(t2457: f64, t25945: f64, t25944: f64, t1426: f64, t25920: f64, t7063: f64, t7286: f64, t2470: f64, t7285: f64, t7289: f64, t2030: f64, t25882: f64, t25885: f64, t25889: f64, t25893: f64, t25896: f64, t25902: f64, t25905: f64, t25909: f64, t25914: f64, t25919: f64, t25921: f64, t25926: f64, t25930: f64, t25934: f64, t25941: f64, t4132: f64, t7279: f64, t7292: f64, t7295: f64, t7298: f64, t7308: f64) -> (f64, f64, f64, f64, f64) {
    let t25946 = t25945 * t2457;
    let t25948 = 0.17135234354032049604e-2_f64 * t25944 * t25946;
    let t25949 = t25920 * t1426;
    let t25950 = t7063 * t25949;
    let t25951 = t25950 * t7286;
    let t25953 = t7285 * t2470;
    let t25955 = 0.17135234354032049604e-1_f64 * t7289 * t25953;
    let t25956 = 0.51405703062096148812e-1_f64 * t25882 + 0.8673628188205199462e0_f64 * t7295 * t25885 + 0.17347256376410398924e1_f64 * t7295 * t25889 + t25893 - 0.28912093960683998208e-1_f64 * t25896 + 0.25702851531048074406e-1_f64 * t25902 - 0.14456046980341999104e-1_f64 * t25905 - 0.65854491829355115987e0_f64 * t7279 * t4132 - 0.4336814094102599731e0_f64 * t25909 * t2030 - 0.10975748638225852664e-1_f64 * t25914 - t25919 + 0.17347256376410398924e1_f64 * t25921 * t7298 - 0.26020884564615598386e1_f64 * t7295 * t25926 - 0.17347256376410398924e1_f64 * t25930 * t25934 - t25941 - 0.8673628188205199462e0_f64 * t7292 * t7308 + t25948 - 0.25702851531048074406e-1_f64 * t25951 + t25955;
    (t25946, t25949, t25950, t25953, t25956)
}
