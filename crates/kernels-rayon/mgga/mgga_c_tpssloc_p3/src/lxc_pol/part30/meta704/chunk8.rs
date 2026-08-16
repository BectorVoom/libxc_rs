//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2305/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2305(t225: f64, t28557: f64, t28565: f64, t100027: f64, t11059: f64, t11065: f64, t1599: f64, t17671: f64, t17732: f64, t18088: f64, t18103: f64, t18111: f64, t1948: f64, t23327: f64, t23601: f64, t25470: f64, t25484: f64, t25485: f64, t25516: f64, t25517: f64, t28596: f64, t28666: f64, t381: f64, t4347: f64, t4540: f64, t6687: f64, t6784: f64, t6786: f64, t6797: f64, t6799: f64, t6800: f64, t82513: f64, t82620: f64, t89204: f64) -> f64 {
    let t100126 = t28557 * t225;
    let t100137 = t28565 * t225;
    let t100147 = -0.16449340668482264365e-1_f64 * t23601 * t82620 * t28666 + 0.49348022005446793095e-1_f64 * t82513 * t89204 * t100027 * t17671 - 0.16449340668482264365e-1_f64 * t6687 * t1599 * t1948 * t381 * t4540 + 0.54831135561607547884e-2_f64 * t6687 * t6784 * t25516 * t4347 + 0.3289868133696452873e-1_f64 * t23601 * t25484 * t25485 * t17732 - 0.27415567780803773942e-2_f64 * t23327 * t100126 * t6786 + 6.0_f64 * t11059 * t28596 * t18111 + 0.16449340668482264365e-1_f64 * t6797 * t6799 * t18088 * t6800 - 0.27415567780803773942e-2_f64 * t23327 * t100137 * t6786 - 0.54831135561607547883e-2_f64 * t23327 * t25470 * t25517 - 6.0_f64 * t11065 * t28596 * t18103;
    t100147
}
