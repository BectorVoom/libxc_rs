//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 895/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk895(t34803: f64, t38866: f64, t38870: f64, t38873: f64, t38874: f64, t38876: f64, t38882: f64, t38887: f64, t38889: f64, t44977: f64, t44982: f64, t44986: f64, t44990: f64, t44994: f64, t44997: f64, t45002: f64, t45004: f64) -> f64 {
    let t45006 = 0.72042316457491791906e-3_f64 * t38866 - t38870 - t38873 - 0.38422568777328955684e-2_f64 * t38874 + 0.92232789896410962678e-3_f64 * t38876 - 0.1333427903096438929e0_f64 * t34803 + 0.19863479950205658386e-4_f64 * t44977 + t38882 + t38887 + 0.16260079888840015101e-2_f64 * t38889 - 0.18183107769496894485e0_f64 * t44982 - 0.15323255961587222183e-3_f64 * t44986 + 0.30646511923174444366e-3_f64 * t44990 + 0.76616279807936110914e-4_f64 * t44994 - 0.76616279807936110914e-4_f64 * t44997 + 0.31923449919973379548e-4_f64 * t45002 + 0.25538759935978703638e-4_f64 * t45004;
    t45006
}
