//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1390/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1390<F: Float>(t14537: F, t2782: F, t10519: F, t10524: F, t10943: F, t14498: F, t14502: F, t14506: F, t14507: F, t14511: F, t14512: F, t14518: F, t14522: F, t14525: F, t14533: F, t4366: F, t4494: F, t4504: F, t4514: F, t837: F) -> F {
    let t14539 = F::cast_from(0.10975748638225852664e-1_f64) * t2782 * t14537;
    let t14540 = F::cast_from(0.2601984143835408805e-1_f64) * t10519 - F::cast_from(0.9757440539382783019e-2_f64) * t10524 + t14498 + F::cast_from(0.13170898365871023197e1_f64) * t4504 * t4494 * t10943 + t14506 + t14511 + F::cast_from(0.65049603595885220126e-3_f64) * t14512 - F::cast_from(0.13170898365871023197e1_f64) * t4514 * t14502 * t837 - t14518 - t14522 - F::cast_from(0.11565819519348392139e-2_f64) * t14525 + F::cast_from(0.26341796731742046394e1_f64) * t4504 * t14502 * t4366 + F::cast_from(0.26341796731742046394e1_f64) * t4504 * t14507 * t4366 - F::cast_from(0.73171657588172351096e-2_f64) * t14533 + t14539;
    t14540
}
