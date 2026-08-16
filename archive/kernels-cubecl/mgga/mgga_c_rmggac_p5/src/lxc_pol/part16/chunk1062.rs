//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1062/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1062<F: Float>(t2211: F, t6463: F, t10353: F, t1356: F, t2604: F, t34659: F, t34662: F, t34665: F, t38312: F, t38314: F, t38318: F, t38322: F, t38326: F, t44580: F, t44584: F, t44590: F, t44595: F, t44600: F, t44605: F, t44610: F) -> (F, F) {
    let t48122 = t2211 * t6463;
    let t48139 = F::cast_from(0.325201597776800302e-2_f64) * t38312 + F::cast_from(0.38422568777328955681e-2_f64) * t38314 - F::cast_from(0.1333427903096438929e0_f64) * t38318 + F::cast_from(0.39914139006212695214e-1_f64) * t1356 * t48122 + F::cast_from(0.66671395154821946452e-1_f64) * t34659 - F::cast_from(0.78064147182743091554e-3_f64) * t38322 + F::cast_from(0.12195059916630011325e-2_f64) * t38326 - F::cast_from(0.59871208509319042821e-1_f64) * t2604 * t10353 + F::cast_from(0.29810146462873361016e-2_f64) * t34662 + F::cast_from(0.29810146462873361016e-2_f64) * t34665 - F::cast_from(0.85129199786595678799e-5_f64) * t44580 + F::cast_from(0.3405167991463827152e-4_f64) * t44584 + F::cast_from(0.1702583995731913576e-4_f64) * t44590 + F::cast_from(0.1064114997332445985e-4_f64) * t44595 - F::cast_from(0.1702583995731913576e-4_f64) * t44600 + F::cast_from(0.5107751987195740728e-4_f64) * t44605 - F::cast_from(0.5107751987195740728e-4_f64) * t44610;
    (t48122, t48139)
}
