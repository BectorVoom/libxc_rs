//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1863/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1863(t1000: f64, t25461: f64, t25476: f64, t25611: f64, t25629: f64, t27412: f64, t27415: f64, t27419: f64, t27423: f64, t27427: f64, t27433: f64, t27437: f64, t27441: f64, t27445: f64, t27545: f64, t27550: f64, t342: f64, t4947: f64, t7140: f64, t7144: f64, t7153: f64, t7159: f64, t7818: f64, t7822: f64) -> f64 {
    let t27553 = 0.8673628188205199462e0_f64 * t25461 * t7822 + 0.8673628188205199462e0_f64 * t7159 * t27412 - 0.8673628188205199462e0_f64 * t27415 * t7818 + 0.8673628188205199462e0_f64 * t27419 * t7153 - 0.8673628188205199462e0_f64 * t7144 * t27423 + 0.8673628188205199462e0_f64 * t7159 * t27427 - 0.8673628188205199462e0_f64 * t25476 * t7818 - 0.8673628188205199462e0_f64 * t25629 * t27433 + 0.8673628188205199462e0_f64 * t25611 * t27437 + 0.8673628188205199462e0_f64 * t7159 * t27441 - 0.8673628188205199462e0_f64 * t7144 * t27445 + 0.65854491829355115987e0_f64 * t342 * t27545 + 0.13170898365871023197e1_f64 * t7140 * t4947 - 0.65854491829355115987e0_f64 * t27550 * t1000;
    t27553
}
