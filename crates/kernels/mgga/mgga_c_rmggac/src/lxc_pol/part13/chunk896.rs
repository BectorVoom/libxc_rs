//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 896/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk896<F: Float>(t8677: F, t8679: F, t8681: F, t8683: F, t8685: F, t8690: F, t9423: F, t9425: F, t10357: F, t10358: F, t10359: F, t10360: F, t8172: F, t9428: F, t8809: F, t8813: F) -> (F, F, F, F, F, F) {
    let t42501 = 0.212822999466489197e-4 * t8677;
    let t42502 = 0.1702583995731913576e-4 * t8679;
    let t42504 = 0.5107751987195740728e-4 * t8681;
    let t42505 = 0.5107751987195740728e-4 * t8683;
    let t42506 = 0.1702583995731913576e-4 * t8685;
    let t42507 = 0.1702583995731913576e-4 * t8690;
    let t42508 = 0.79828278012425390428e-1 * t9423;
    let t42509 = 0.39914139006212695214e-1 * t9425;
    let t42510 = t42504 - t42505 - t42506 + t42507 + t10357 + t10358 - t10359 + t10360 + t42508 - t42509 - t8172;
    let t42515 = 0.79828278012425390428e-1 * t9428;
    let t42516 = 0.20431007948782962912e-3 * t8809;
    let t42517 = 0.5107751987195740728e-4 * t8813;
    (t42501, t42502, t42510, t42515, t42516, t42517)
}
