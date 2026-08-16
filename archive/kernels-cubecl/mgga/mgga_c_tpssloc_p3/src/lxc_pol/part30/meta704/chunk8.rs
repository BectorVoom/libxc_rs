//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2305/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2305<F: Float>(t225: F, t28557: F, t28565: F, t100027: F, t11059: F, t11065: F, t1599: F, t17671: F, t17732: F, t18088: F, t18103: F, t18111: F, t1948: F, t23327: F, t23601: F, t25470: F, t25484: F, t25485: F, t25516: F, t25517: F, t28596: F, t28666: F, t381: F, t4347: F, t4540: F, t6687: F, t6784: F, t6786: F, t6797: F, t6799: F, t6800: F, t82513: F, t82620: F, t89204: F) -> F {
    let t100126 = t28557 * t225;
    let t100137 = t28565 * t225;
    let t100147 = -F::cast_from(0.16449340668482264365e-1_f64) * t23601 * t82620 * t28666 + F::cast_from(0.49348022005446793095e-1_f64) * t82513 * t89204 * t100027 * t17671 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t1599 * t1948 * t381 * t4540 + F::cast_from(0.54831135561607547884e-2_f64) * t6687 * t6784 * t25516 * t4347 + F::cast_from(0.3289868133696452873e-1_f64) * t23601 * t25484 * t25485 * t17732 - F::cast_from(0.27415567780803773942e-2_f64) * t23327 * t100126 * t6786 + F::cast_from(6.0_f64) * t11059 * t28596 * t18111 + F::cast_from(0.16449340668482264365e-1_f64) * t6797 * t6799 * t18088 * t6800 - F::cast_from(0.27415567780803773942e-2_f64) * t23327 * t100137 * t6786 - F::cast_from(0.54831135561607547883e-2_f64) * t23327 * t25470 * t25517 - F::cast_from(6.0_f64) * t11065 * t28596 * t18103;
    t100147
}
