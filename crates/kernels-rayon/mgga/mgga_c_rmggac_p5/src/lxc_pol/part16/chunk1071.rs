//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1071/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1071(t34803: f64, t38846: f64, t38850: f64, t38866: f64, t38874: f64, t38876: f64, t38889: f64, t42759: f64, t42760: f64, t42761: f64, t42762: f64, t42764: f64, t42767: f64, t42771: f64, t42772: f64, t44977: f64, t44982: f64) -> f64 {
    let t48307 = -0.17347588262831798123e-3_f64 * t38846 - 0.2881692658299671676e-2_f64 * t38850 - t42759 + t42760 + t42761 + t42762 + 0.1440846329149835838e-2_f64 * t38866 - t42764 - t42767 - 0.7684513755465791136e-2_f64 * t38874 + 0.18446557979282192534e-2_f64 * t38876 - 0.26668558061928778581e0_f64 * t34803 + 0.39726959900411316773e-4_f64 * t44977 + t42771 + t42772 + 0.325201597776800302e-2_f64 * t38889 - 0.36366215538993788973e0_f64 * t44982;
    t48307
}
