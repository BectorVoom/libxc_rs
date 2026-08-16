//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 929/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk929(t3692: f64, t1307: f64, t1388: f64, t2408: f64, t2417: f64, t2423: f64, t3686: f64, t3688: f64, t3690: f64, t3695: f64, t3813: f64, t3918: f64, t5122: f64, t5126: f64, t5127: f64, t5131: f64, t5153: f64, t5156: f64, t5159: f64, t5160: f64, t5161: f64) -> (f64, f64) {
    let t5164 = 0.5848223622634646207e0_f64 * t3692;
    let t5165 = 3.0_f64 * t1307 * t3918 * t5122 + 6.0_f64 * t1307 * t5126 * t5127 - t1388 * t5160 * t5161 + 3.0_f64 * t3918 * t5131 + t2408 + t2417 - t2423 + t3686 + t3688 - t3690 - t3695 + t3813 + t5153 - t5156 - t5159 - t5164;
    (t5164, t5165)
}
