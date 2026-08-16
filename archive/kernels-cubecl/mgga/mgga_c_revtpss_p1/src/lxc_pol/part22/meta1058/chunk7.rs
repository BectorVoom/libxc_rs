//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3760/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3760<F: Float>(t20913: F, t3172: F, t3711: F, t3147: F, t6593: F, t3594: F, t3597: F, t1244: F, t1042: F, t1222: F, t17500: F, t17541: F, t17569: F, t17584: F, t17700: F, t20982: F, t20986: F, t21102: F, t3591: F, t3606: F, t3613: F, t3647: F, t5056: F, t5299: F, t5308: F, t5391: F, t57053: F, t68299: F, t68303: F) -> F {
    let t71687 = t3711 * t3172 * t20913;
    let t71691 = t6593 * t3147;
    let t71693 = t3594 * t3597 * t71691;
    let t71699 = t3594 * t1244 * t71691;
    let t71704 = -t1222 * t5308 * t68299 / F::cast_from(144.0_f64) - t1222 * t5308 * t68303 / F::cast_from(48.0_f64) + F::cast_from(0.57165357490759649296e-3_f64) * t3711 * t1042 * t17500 * t5056 - F::cast_from(0.11433071498151929859e-2_f64) * t3647 * t20982 - F::cast_from(0.17149607247227894789e-2_f64) * t3647 * t20986 + F::cast_from(0.57165357490759649296e-3_f64) * t57053 * t5299 - F::cast_from(0.5081365110289746604e-2_f64) * t5391 * t17700 + F::cast_from(0.3811023832717309953e-3_f64) * t71687 + F::cast_from(0.72409452821628889107e-2_f64) * t21102 * t3591 + F::cast_from(0.14481890564325777821e-1_f64) * t71693 * t3606 + F::cast_from(0.28582678745379824648e-3_f64) * t17569 * t17541 - F::cast_from(0.72409452821628889107e-2_f64) * t71699 * t3613 + F::cast_from(0.28582678745379824648e-3_f64) * t17569 * t17584;
    t71704
}
