//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 895/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk895<F: Float>(t34803: F, t38866: F, t38870: F, t38873: F, t38874: F, t38876: F, t38882: F, t38887: F, t38889: F, t44977: F, t44982: F, t44986: F, t44990: F, t44994: F, t44997: F, t45002: F, t45004: F) -> F {
    let t45006 = F::new(0.72042316457491791906e-3) * t38866 - t38870 - t38873 - F::new(0.38422568777328955684e-2) * t38874 + F::new(0.92232789896410962678e-3) * t38876 - F::new(0.1333427903096438929e0) * t34803 + F::new(0.19863479950205658386e-4) * t44977 + t38882 + t38887 + F::new(0.16260079888840015101e-2) * t38889 - F::new(0.18183107769496894485e0) * t44982 - F::new(0.15323255961587222183e-3) * t44986 + F::new(0.30646511923174444366e-3) * t44990 + F::new(0.76616279807936110914e-4) * t44994 - F::new(0.76616279807936110914e-4) * t44997 + F::new(0.31923449919973379548e-4) * t45002 + F::new(0.25538759935978703638e-4) * t45004;
    t45006
}
