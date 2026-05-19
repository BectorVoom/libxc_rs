//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1130/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1130<F: Float>(t1988: F, t8486: F, t1967: F, t8838: F, t4352: F, t535: F, t598: F, t7656: F, t1089: F, t12473: F, t2288: F, t31251: F, t31256: F, t31259: F, t35485: F, t35486: F, t35490: F, t35494: F, t35497: F, t35499: F, t35503: F, t35506: F, t35507: F, t35508: F, t35511: F) -> F {
    let t35513 = t1988 * t8486;
    let t35514 = F::cast_from(0.94344276868812456204e-3_f64) * t35513;
    let t35515 = t1967 * t8838;
    let t35519 = t598 * t4352 * t535 * t7656;
    let t35523 = t598 * t1089 * t12473 * t2288;
    let t35525 = -t35485 - F::cast_from(0.12862205435420921092e-2_f64) * t35486 - F::cast_from(0.10718504529517434243e-3_f64) * t35490 + F::cast_from(0.21437009059034868486e-3_f64) * t35494 + t35497 + F::cast_from(0.31448092289604152067e-3_f64) * t31251 - t35499 - t35503 - F::cast_from(0.42874018118069736972e-3_f64) * t31256 + F::new(0.39221875e0) * t31259 + t35506 - t35507 - t35508 + F::cast_from(0.94344276868812456204e-3_f64) * t35511 + t35514 + F::cast_from(0.64311027177104605458e-2_f64) * t35515 + F::cast_from(0.32155513588552302729e-2_f64) * t35519 - F::cast_from(0.21437009059034868486e-3_f64) * t35523;
    t35525
}
