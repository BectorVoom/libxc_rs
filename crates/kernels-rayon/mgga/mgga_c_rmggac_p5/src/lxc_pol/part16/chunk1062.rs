//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1062/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1062(t2211: f64, t6463: f64, t10353: f64, t1356: f64, t2604: f64, t34659: f64, t34662: f64, t34665: f64, t38312: f64, t38314: f64, t38318: f64, t38322: f64, t38326: f64, t44580: f64, t44584: f64, t44590: f64, t44595: f64, t44600: f64, t44605: f64, t44610: f64) -> (f64, f64) {
    let t48122 = t2211 * t6463;
    let t48139 = 0.325201597776800302e-2_f64 * t38312 + 0.38422568777328955681e-2_f64 * t38314 - 0.1333427903096438929e0_f64 * t38318 + 0.39914139006212695214e-1_f64 * t1356 * t48122 + 0.66671395154821946452e-1_f64 * t34659 - 0.78064147182743091554e-3_f64 * t38322 + 0.12195059916630011325e-2_f64 * t38326 - 0.59871208509319042821e-1_f64 * t2604 * t10353 + 0.29810146462873361016e-2_f64 * t34662 + 0.29810146462873361016e-2_f64 * t34665 - 0.85129199786595678799e-5_f64 * t44580 + 0.3405167991463827152e-4_f64 * t44584 + 0.1702583995731913576e-4_f64 * t44590 + 0.1064114997332445985e-4_f64 * t44595 - 0.1702583995731913576e-4_f64 * t44600 + 0.5107751987195740728e-4_f64 * t44605 - 0.5107751987195740728e-4_f64 * t44610;
    (t48122, t48139)
}
