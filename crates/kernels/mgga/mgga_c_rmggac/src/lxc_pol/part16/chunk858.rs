//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 858/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk858<F: Float>(t9410: F, t8677: F, t8679: F, t8681: F, t8683: F, t8685: F, t8690: F, t9423: F, t9425: F, t9428: F, t8809: F, t8813: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t42500 = F::cast_from(0.11974241701863808564e0_f64) * t9410;
    let t42501 = F::cast_from(0.212822999466489197e-4_f64) * t8677;
    let t42502 = F::cast_from(0.1702583995731913576e-4_f64) * t8679;
    let t42504 = F::cast_from(0.5107751987195740728e-4_f64) * t8681;
    let t42505 = F::cast_from(0.5107751987195740728e-4_f64) * t8683;
    let t42506 = F::cast_from(0.1702583995731913576e-4_f64) * t8685;
    let t42507 = F::cast_from(0.1702583995731913576e-4_f64) * t8690;
    let t42508 = F::cast_from(0.79828278012425390428e-1_f64) * t9423;
    let t42509 = F::cast_from(0.39914139006212695214e-1_f64) * t9425;
    let t42515 = F::cast_from(0.79828278012425390428e-1_f64) * t9428;
    let t42516 = F::cast_from(0.20431007948782962912e-3_f64) * t8809;
    let t42517 = F::cast_from(0.5107751987195740728e-4_f64) * t8813;
    (t42500, t42501, t42502, t42504, t42505, t42506, t42507, t42508, t42509, t42515, t42516, t42517)
}
