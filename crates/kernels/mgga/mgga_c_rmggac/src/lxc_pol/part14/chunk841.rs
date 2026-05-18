//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 841/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk841<F: Float>(t29892: F, t3351: F, t3352: F, t515: F, t2010: F, t2012: F, t5061: F, t34705: F, t34707: F, t34711: F, t34713: F, t34717: F, t38695: F, t38699: F, t38702: F, t38705: F, t38708: F, t38710: F, t38712: F, t38717: F, t38719: F, t38724: F, t4985: F, t7564: F) -> F {
    let t38728 = t3351 * t3352 * t515 * t29892;
    let t38733 = t2010 * t2012 * t5061;
    let t38735 = -F::new(0.8980681276397856423e-1) * t38695 + t34705 + t34707 - t34711 - F::new(0.51240438831339423711e-4) * t34713 + F::new(0.72042316457491791906e-3) * t34717 - F::new(0.85129199786595678796e-5) * t38699 + F::new(0.85129199786595678796e-5) * t38702 + t38705 - F::new(0.76616279807936110914e-4) * t38708 - F::new(0.23836175940246790062e-3) * t38710 - F::new(0.59590439850616975156e-4) * t38712 - F::new(0.25538759935978703638e-4) * t38717 - F::new(0.25538759935978703638e-4) * t38719 + F::new(0.85129199786595678796e-5) * t38724 - F::new(0.25538759935978703638e-4) * t38728 + F::new(0.11974241701863808564e0) * t4985 * t7564 - F::new(0.72042316457491791906e-3) * t38733;
    t38735
}
