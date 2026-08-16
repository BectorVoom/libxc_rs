//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3104/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3104<F: Float>(t12772: F, t17645: F, t3625: F, t1284: F, t17288: F, t3624: F, t12917: F, t17401: F, t1121: F, t11231: F, t12732: F, t12855: F, t12862: F, t12876: F, t13046: F, t16714: F, t16756: F, t17353: F, t17454: F, t17456: F, t17609: F, t17654: F, t17655: F, t21017: F, t2258: F, t3588: F, t3591: F, t3604: F, t3720: F, t3723: F, t44225: F, t44291: F, t44293: F, t44326: F, t44484: F, t45764: F, t471: F, t5330: F, t5331: F, t5332: F, t5335: F, t56997: F, t56999: F, t57005: F, t57021: F, t57026: F, t606: F) -> F {
    let t57029 = t3625 * t12772 * t17645;
    let t57040 = t17288 * t1284 * t3624;
    let t57045 = t17401 * t12917;
    let t57047 = -F::cast_from(0.85748036236139473944e-3_f64) * t17654 * t17353 * t3604 * t3588 * t1121 * t606 - F::cast_from(0.85748036236139473944e-3_f64) * t17654 * t17353 * t3604 * t17655 * t2258 - F::cast_from(0.25724410870841842184e-2_f64) * t56997 * t17353 * t13046 * t56999 - F::cast_from(0.19055119163586549765e-2_f64) * t57005 * t44225 * t16714 * t11231 - F::cast_from(0.21437009059034868486e-3_f64) * t5331 * t3720 * t5332 * t12732 * t471 - F::cast_from(0.64311027177104605458e-3_f64) * t45764 * t5330 * t5335 + F::cast_from(0.19055119163586549765e-3_f64) * t44291 - F::cast_from(0.14291339372689912324e-3_f64) * t44293 + F::cast_from(0.14291339372689912324e-3_f64) * t44326 + F::cast_from(0.17149607247227894789e-2_f64) * t57021 + F::cast_from(0.64311027177104605458e-3_f64) * t17609 * t3591 - F::cast_from(0.28582678745379824648e-3_f64) * t57026 - F::cast_from(0.57165357490759649295e-3_f64) * t57029 - F::cast_from(0.25724410870841842183e-2_f64) * t44484 * t17456 - F::cast_from(0.25724410870841842183e-2_f64) * t12855 * t3720 * t16756 * t17454 + F::cast_from(0.34299214494455789577e-2_f64) * t21017 * t12876 - F::cast_from(0.12862205435420921092e-2_f64) * t57040 * t3723 - F::cast_from(0.64311027177104605458e-3_f64) * t17401 * t12862 - F::cast_from(0.85748036236139473944e-3_f64) * t57045;
    t57047
}
