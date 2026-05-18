//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 913/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk913<F: Float>(t16043: F, t9096: F, t1971: F, t2144: F, t27044: F, t3351: F, t2604: F, t35327: F, t35337: F, t39715: F, t39717: F, t39721: F, t39726: F, t39731: F, t39733: F, t39735: F, t39742: F, t39748: F, t39752: F, t39754: F, t39756: F, t39758: F, t8378: F) -> F {
    let t39760 = t16043 * t9096;
    let t39764 = t3351 * t1971 * t2144 * t27044;
    let t39766 = F::new(0.12769379967989351819e-4) * t39715 - F::new(0.12769379967989351819e-4) * t39717 - F::new(0.25538759935978703638e-4) * t39721 + F::new(0.12769379967989351819e-4) * t39726 + F::new(0.42564599893297839398e-5) * t39731 - F::new(0.42564599893297839398e-5) * t39733 + F::new(0.85129199786595678796e-5) * t39735 + F::new(0.23948483403727617128e0) * t2604 * t8378 - F::new(0.12769379967989351819e-3) * t39742 - F::new(0.66211599834018861286e-4) * t35327 - F::new(0.59590439850616975158e-4) * t35337 + F::new(0.51077519871957407276e-4) * t39748 - F::new(0.76616279807936110914e-4) * t39752 - F::new(0.85129199786595678796e-5) * t39754 - F::new(0.53205749866622299248e-5) * t39756 - F::new(0.1064114997332445985e-4) * t39758 + F::new(0.25538759935978703638e-4) * t39760 + F::new(0.25538759935978703638e-4) * t39764;
    t39766
}
