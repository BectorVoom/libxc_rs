//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2823/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2823<F: Float>(t10811: F, t23323: F, t14494: F, t14785: F, t14786: F, t14791: F, t14894: F, t18616: F, t18637: F, t2745: F, t2747: F, t2749: F, t36833: F, t40361: F, t4362: F, t4364: F, t4365: F, t4366: F, t4433: F, t50299: F, t50757: F, t5978: F, t6017: F, t61532: F, t76194: F, t76279: F, t76284: F, t76289: F, t76302: F, t76313: F, t76315: F, t76321: F, t76330: F, t837: F) -> F {
    let t76337 = t10811 * t23323;
    let t76343 = F::cast_from(0.85748036236139473944e-3_f64) * t2745 * t2747 * t76279 * t837 + F::cast_from(0.51448821741683684368e-2_f64) * t50757 * t4364 * t76284 * t76194 + F::cast_from(0.85748036236139473944e-3_f64) * t2745 * t2747 * t76289 * t2749 - F::cast_from(0.21437009059034868486e-3_f64) * t2745 * t4364 * t76289 * t837 - F::cast_from(0.64311027177104605458e-3_f64) * t2745 * t36833 * t14494 * t6017 - F::cast_from(0.51448821741683684366e-2_f64) * t4362 * t14791 * t76302 * t4366 - F::cast_from(0.64311027177104605458e-3_f64) * t2745 * t4364 * t4365 * t18616 - F::cast_from(0.38115002106963996169e-4_f64) * t76313 - F::cast_from(0.12004725073059526352e-1_f64) * t76315 - F::cast_from(0.12862205435420921092e-1_f64) * t2745 * t14785 * t5978 * t4433 - F::cast_from(0.25724410870841842184e-1_f64) * t2745 * t14785 * t76321 * t18637 + F::cast_from(0.42874018118069736972e-3_f64) * t4362 * t4364 * t76289 * t4366 - F::cast_from(0.12004725073059526352e-1_f64) * t76330 + F::cast_from(0.37792653007779990369e-1_f64) * t40361 - t50299 + F::cast_from(0.1543464652250510531e-1_f64) * t14894 * t14791 * t61532 * t14786 - F::cast_from(0.12004725073059526352e-1_f64) * t76337 + F::cast_from(0.85748036236139473944e-3_f64) * t2745 * t2747 * t76284 * t2749;
    t76343
}
