//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1063/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1063<F: Float>(t39873: F, t39899: F, t35473: F, t35478: F, t35481: F, t35484: F, t35487: F, t35514: F, t35516: F, t37720: F, t37731: F, t39871: F, t39877: F, t39881: F, t39889: F, t39893: F, t39901: F, t39907: F) -> F {
    let t43169 = F::new(0.39726959900411316772e-4) * t39873;
    let t43179 = F::new(0.10909864661698136692e0) * t39899;
    let t43184 = F::new(0.5107751987195740728e-4) * t39871 + t43169 + F::new(0.40911992481368012596e-1) * t39877 - F::new(0.81823984962736025192e-1) * t39881 + t37720 + F::new(0.19863479950205658386e-4) * t35473 + F::new(0.85129199786595678799e-5) * t39889 + F::new(0.23942587439980034662e-4) * t39893 + F::new(0.325201597776800302e-2) * t35478 - F::new(0.78064147182743091556e-3) * t35481 + F::new(0.325201597776800302e-2) * t35484 - F::new(0.78064147182743091556e-3) * t35487 + t37731 - t43179 + F::new(0.19863479950205658387e-3) * t39901 + F::new(0.13242319966803772257e-3) * t35514 + F::new(0.39726959900411316772e-4) * t35516 - F::new(0.20431007948782962912e-3) * t39907;
    t43184
}
