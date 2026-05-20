//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3107/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3107<F: Float>(t17861: F, t3624: F, t1042: F, t1260: F, t1261: F, t12646: F, t12920: F, t13055: F, t13076: F, t16714: F, t1715: F, t17202: F, t17429: F, t17729: F, t17736: F, t17786: F, t3606: F, t3626: F, t3631: F, t3714: F, t44225: F, t44421: F, t5051: F, t5293: F, t53450: F, t5386: F, t57049: F, t57053: F, t57056: F, t57063: F, t57065: F, t57070: F, t57075: F, t57077: F, t57083: F, t57094: F, t57098: F) -> F {
    let t57100 = t17861 * t3624;
    let t57103 = F::cast_from(0.45732285992607719436e-2_f64) * t57049 - F::cast_from(0.11433071498151929859e-2_f64) * t5293 * t13076 + F::cast_from(0.85748036236139473944e-3_f64) * t57053 * t3714 - F::cast_from(0.68598428988911579154e-2_f64) * t57056 * t3606 - F::cast_from(0.25724410870841842183e-2_f64) * t1261 * t1042 * t17202 * t53450 + F::cast_from(0.85748036236139473944e-3_f64) * t57063 - F::cast_from(0.12862205435420921092e-2_f64) * t57065 * t13055 + F::cast_from(0.17149607247227894789e-2_f64) * t57070 + F::cast_from(0.12862205435420921092e-2_f64) * t44421 * t1260 * t5386 - F::cast_from(0.57165357490759649295e-3_f64) * t57075 + F::cast_from(0.15244095330869239812e-2_f64) * t57077 - F::cast_from(0.85748036236139473944e-3_f64) * t17736 * t3626 * t1715 * t12646 - F::cast_from(0.17149607247227894789e-2_f64) * t17736 * t3626 * t5051 * t57083 + F::cast_from(0.19055119163586549765e-2_f64) * t17729 * t44225 * t16714 * t12920 - F::cast_from(0.64311027177104605458e-3_f64) * t17429 * t17786 + F::cast_from(0.95275595817932748825e-4_f64) * t57094 + F::cast_from(0.57165357490759649295e-3_f64) * t57098 - F::cast_from(0.85748036236139473944e-3_f64) * t57100 * t3631;
    t57103
}
