//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 909/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk909<F: Float>(t225: F, t27543: F, t385: F, t7810: F, t994: F, t1000: F, t25461: F, t25476: F, t25611: F, t25629: F, t27412: F, t27415: F, t27419: F, t27423: F, t27427: F, t27433: F, t27437: F, t27441: F, t27445: F, t342: F, t4947: F, t7140: F, t7144: F, t7153: F, t7159: F, t7818: F, t7822: F) -> F {
    let t27545 = t27543 * t225 * t385;
    let t27550 = t994 * t7810;
    let t27553 = F::new(0.8673628188205199462e0) * t25461 * t7822 + F::new(0.8673628188205199462e0) * t7159 * t27412 - F::new(0.8673628188205199462e0) * t27415 * t7818 + F::new(0.8673628188205199462e0) * t27419 * t7153 - F::new(0.8673628188205199462e0) * t7144 * t27423 + F::new(0.8673628188205199462e0) * t7159 * t27427 - F::new(0.8673628188205199462e0) * t25476 * t7818 - F::new(0.8673628188205199462e0) * t25629 * t27433 + F::new(0.8673628188205199462e0) * t25611 * t27437 + F::new(0.8673628188205199462e0) * t7159 * t27441 - F::new(0.8673628188205199462e0) * t7144 * t27445 + F::new(0.65854491829355115987e0) * t342 * t27545 + F::new(0.13170898365871023197e1) * t7140 * t4947 - F::new(0.65854491829355115987e0) * t27550 * t1000;
    t27553
}
