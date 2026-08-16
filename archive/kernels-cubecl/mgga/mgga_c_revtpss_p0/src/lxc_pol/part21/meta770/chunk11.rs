//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2737/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2737<F: Float>(t10811: F, t14682: F, t14804: F, t14923: F, t10777: F, t10779: F, t4457: F, t837: F, t14853: F, t2652: F, t125: F, t14468: F, t14676: F, t14767: F, t14785: F, t14872: F, t2745: F, t2747: F, t2754: F, t40367: F, t40374: F, t40376: F, t40381: F, t40385: F, t40390: F, t40393: F, t40395: F, t40399: F, t40403: F, t4364: F) -> F {
    let t50328 = t10811 * t14682;
    let t50347 = t14923 * t14804;
    let t50351 = t10777 * t10779 * t4457 * t837;
    let t50353 = t2652 * t14853;
    let t50365 = F::cast_from(0.30011812682648815881e-2_f64) * t50328 + F::cast_from(0.25724410870841842183e-2_f64) * t2745 * t2747 * t14676 * t14872 - F::cast_from(0.64311027177104605458e-3_f64) * t2745 * t4364 * t14676 * t2754 + F::cast_from(0.25724410870841842183e-2_f64) * t2745 * t2747 * t125 * t14468 * t837 - F::cast_from(0.12862205435420921092e-1_f64) * t2745 * t14785 * t14767 * t837 + F::cast_from(0.48018900292238105409e-1_f64) * t50347 + F::cast_from(0.30492001685571196935e-3_f64) * t50351 + F::cast_from(0.12004725073059526352e-1_f64) * t50353 - F::cast_from(0.60023625365297631762e-1_f64) * t40367 + F::cast_from(0.40656002247428262581e-3_f64) * t40374 + F::cast_from(0.10003937560882938627e-2_f64) * t40376 + F::cast_from(0.17149607247227894789e-3_f64) * t40381 - F::cast_from(0.42874018118069736972e-4_f64) * t40385 - F::cast_from(0.85748036236139473944e-4_f64) * t40390 - F::cast_from(0.17006693853500995666e-1_f64) * t40393 - F::cast_from(0.17006693853500995666e-1_f64) * t40395 + F::cast_from(0.34013387707001991333e-1_f64) * t40399 + F::cast_from(0.76230004213927992336e-4_f64) * t40403;
    t50365
}
