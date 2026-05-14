//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 992/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk992<F: Float>(t28476: F, t303: F, t3964: F, t6140: F, t1385: F, t1650: F, t27356: F, t5709: F, t27453: F, t5654: F, t27438: F, t5701: F, t2237: F, t2239: F, t27369: F, t27450: F, t27462: F, t27471: F, t28348: F, t28353: F, t28403: F, t28443: F, t28451: F, t28454: F, t28461: F, t28465: F, t28467: F, t28471: F, t28474: F, t7908: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28477 = t303 * t28476;
    let t28480 = t3964 * t6140;
    let t28483 = t1650 * t1385;
    let t28484 = t27356 * t28483;
    let t28485 = t5709 * t28484;
    let t28488 = t27453 * t5654;
    let t28489 = t5709 * t28488;
    let t28494 = t27438 * t5654;
    let t28495 = t5701 * t28494;
    let t28498 = 0.30918233506944444444e-4 * t27369 * t28443 - 0.16581944444444444444e-2 * t28451 + 0.11054629629629629629e-2 * t28454 - 0.23168402777777777778e-3 * t27450 - 0.92754700520833333333e-4 * t27369 * t28348 - 0.2782641015625e-3 * t27369 * t28353 - 0.69505208333333333333e-3 * t28461 * t2239 + 0.16581944444444444444e-2 * t27462 + 0.16581944444444444444e-2 * t28465 - 0.23168402777777777778e-3 * t28467 + 0.69505208333333333333e-3 * t2237 * t28403 + 0.61782407407407407407e-3 * t28471 + 0.24872916666666666666e-2 * t28474 - 0.66327777777777777776e-2 * t28477 + 0.30918233506944444445e-4 * t27471 + 0.18534722222222222222e-2 * t28480 * t2239 + 0.23168402777777777778e-3 * t7908 * t28485 + 0.46336805555555555556e-3 * t7908 * t28489 + 0.30918233506944444445e-4 * t27369 * t28485 - 0.30891203703703703704e-3 * t7908 * t28495;
    (t28477, t28480, t28484, t28485, t28488, t28489, t28494, t28495, t28498)
}
