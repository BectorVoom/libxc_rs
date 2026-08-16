//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 852/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk852(t3805: f64, t3807: f64, t5249: f64, t2408: f64, t2417: f64, t2423: f64, t3686: f64, t3688: f64, t3690: f64, t3695: f64, t3813: f64, t5153: f64, t5156: f64, t5159: f64, t5164: f64, t5167: f64) -> (f64, f64) {
    let t5259 = t3805 * t5249 * t3807;
    let t5262 = t3686 + t5153 - t5156 - t5159 + t3688 - t3690 - t5164 - t3695 + t3813 + t2408 + t2417 - t2423 + t5167;
    (t5259, t5262)
}
