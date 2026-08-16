//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1863/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1863<F: Float>(t1000: F, t25461: F, t25476: F, t25611: F, t25629: F, t27412: F, t27415: F, t27419: F, t27423: F, t27427: F, t27433: F, t27437: F, t27441: F, t27445: F, t27545: F, t27550: F, t342: F, t4947: F, t7140: F, t7144: F, t7153: F, t7159: F, t7818: F, t7822: F) -> F {
    let t27553 = F::cast_from(0.8673628188205199462e0_f64) * t25461 * t7822 + F::cast_from(0.8673628188205199462e0_f64) * t7159 * t27412 - F::cast_from(0.8673628188205199462e0_f64) * t27415 * t7818 + F::cast_from(0.8673628188205199462e0_f64) * t27419 * t7153 - F::cast_from(0.8673628188205199462e0_f64) * t7144 * t27423 + F::cast_from(0.8673628188205199462e0_f64) * t7159 * t27427 - F::cast_from(0.8673628188205199462e0_f64) * t25476 * t7818 - F::cast_from(0.8673628188205199462e0_f64) * t25629 * t27433 + F::cast_from(0.8673628188205199462e0_f64) * t25611 * t27437 + F::cast_from(0.8673628188205199462e0_f64) * t7159 * t27441 - F::cast_from(0.8673628188205199462e0_f64) * t7144 * t27445 + F::cast_from(0.65854491829355115987e0_f64) * t342 * t27545 + F::cast_from(0.13170898365871023197e1_f64) * t7140 * t4947 - F::cast_from(0.65854491829355115987e0_f64) * t27550 * t1000;
    t27553
}
