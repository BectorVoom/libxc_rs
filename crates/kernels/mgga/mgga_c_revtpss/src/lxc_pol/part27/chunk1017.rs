//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1017/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1017<F: Float>(t25950: F, t7286: F, t2470: F, t7285: F, t7289: F, t2030: F, t25882: F, t25885: F, t25889: F, t25893: F, t25896: F, t25902: F, t25905: F, t25909: F, t25914: F, t25919: F, t25921: F, t25926: F, t25930: F, t25934: F, t25941: F, t25948: F, t4132: F, t7279: F, t7292: F, t7295: F, t7298: F, t7308: F) -> (F, F) {
    let t25951 = t25950 * t7286;
    let t25953 = t7285 * t2470;
    let t25955 = 0.17135234354032049604e-1 * t7289 * t25953;
    let t25956 = 0.51405703062096148812e-1 * t25882 + 0.8673628188205199462e0 * t7295 * t25885 + 0.17347256376410398924e1 * t7295 * t25889 + t25893 - 0.28912093960683998208e-1 * t25896 + 0.25702851531048074406e-1 * t25902 - 0.14456046980341999104e-1 * t25905 - 0.65854491829355115987e0 * t7279 * t4132 - 0.4336814094102599731e0 * t25909 * t2030 - 0.10975748638225852664e-1 * t25914 - t25919 + 0.17347256376410398924e1 * t25921 * t7298 - 0.26020884564615598386e1 * t7295 * t25926 - 0.17347256376410398924e1 * t25930 * t25934 - t25941 - 0.8673628188205199462e0 * t7292 * t7308 + t25948 - 0.25702851531048074406e-1 * t25951 + t25955;
    (t25953, t25956)
}
