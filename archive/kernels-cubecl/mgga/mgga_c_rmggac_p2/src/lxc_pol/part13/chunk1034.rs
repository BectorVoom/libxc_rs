//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1034/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1034<F: Float>(t9124: F, t9637: F, t34659: F, t34662: F, t34665: F, t38312: F, t38314: F, t38318: F, t38322: F, t38326: F, t38328: F, t38336: F, t38340: F, t38344: F, t38348: F, t38352: F, t38356: F, t38358: F, t38361: F) -> (F, F, F) {
    let t42574 = F::cast_from(0.212822999466489197e-4_f64) * t9124;
    let t42575 = F::cast_from(0.79828278012425390428e-1_f64) * t9637;
    let t42593 = F::cast_from(0.162600798888400151e-2_f64) * t38312 + F::cast_from(0.3842256877732895568e-2_f64) * t38314 - F::cast_from(0.66671395154821946452e-1_f64) * t38318 + F::cast_from(0.1333427903096438929e0_f64) * t34659 - F::cast_from(0.39032073591371545778e-3_f64) * t38322 + F::cast_from(0.60975299583150056624e-3_f64) * t38326 - F::cast_from(0.11974241701863808564e0_f64) * t38328 + F::cast_from(0.59620292925746722032e-2_f64) * t34662 + F::cast_from(0.59620292925746722032e-2_f64) * t34665 + F::cast_from(0.30487649791575028312e-3_f64) * t38336 + F::cast_from(0.30487649791575028312e-3_f64) * t38340 + F::cast_from(0.60975299583150056624e-3_f64) * t38344 + F::cast_from(0.30487649791575028312e-3_f64) * t38348 - F::cast_from(0.1702583995731913576e-4_f64) * t38352 - F::cast_from(0.1702583995731913576e-4_f64) * t38356 - F::cast_from(0.85129199786595678799e-5_f64) * t38358 + F::cast_from(0.5107751987195740728e-4_f64) * t38361;
    (t42574, t42575, t42593)
}
