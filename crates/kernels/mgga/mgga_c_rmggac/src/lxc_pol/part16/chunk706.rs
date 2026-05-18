//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 706/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk706<F: Float>(t10252: F, t1550: F, t9732: F, t9737: F, t1756: F, t2211: F, t1356: F, t570: F, t9530: F, t9740: F, t1707: F, t699: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10253 = t1550 * t10252;
    let t10254 = F::new(0.11974241701863808564e0) * t10253;
    let t10255 = F::new(0.85129199786595678799e-5) * t9732;
    let t10256 = F::new(0.1702583995731913576e-4) * t9737;
    let t10257 = t2211 * t1756;
    let t10258 = t1356 * t10257;
    let t10259 = F::new(0.39914139006212695214e-1) * t10258;
    let t10260 = t9530 * t570;
    let t10261 = t1356 * t10260;
    let t10262 = F::new(0.79828278012425390428e-1) * t10261;
    let t10263 = F::new(0.17961362552795712846e0) * t9740;
    let t10267 = t699 * t1707;
    (t10254, t10255, t10256, t10257, t10259, t10260, t10262, t10263, t10267)
}
