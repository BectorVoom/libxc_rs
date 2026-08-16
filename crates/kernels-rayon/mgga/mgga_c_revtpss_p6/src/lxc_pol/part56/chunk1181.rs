//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1181/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1181(t3736: f64, t42859: f64, t13038: f64, t1794: f64, t8931: f64, t3153: f64, t1042: f64, t124560: f64, t124621: f64, t124717: f64, t124802: f64, t124931: f64, t1252: f64, t131556: f64, t131576: f64, t131578: f64, t131584: f64, t17459: f64, t1795: f64, t21119: f64, t29279: f64, t31993: f64, t32015: f64, t33425: f64, t33509: f64, t371: f64, t3719: f64, t372: f64, t482: f64, t5236: f64, t5428: f64, t5465: f64, t5497: f64, t8938: f64) -> (f64, f64, f64, f64) {
    let t131591 = t42859 * t3736;
    let t131592 = t131591 * t13038;
    let t131594 = t8931 * t1794;
    let t131595 = t131594 * t3153;
    let t131599 = 0.19833242244827205771e-2_f64 * t131556 * t1252 + 0.3718732920905101082e-3_f64 * t33509 * t371 * t372 * t482 * t5497 - 0.11156198762715303246e-2_f64 * t124560 * t1042 * t1795 * t21119 + 0.11156198762715303246e-2_f64 * t124802 * t1042 * t1795 * t17459 + 0.34694512752820797848e1_f64 * t124931 * t29279 - 0.37645955677973955999e-3_f64 * t131576 + 0.56468933516960933998e-3_f64 * t33425 * t32015 * t131578 * t5428 + 0.82638509353446690713e-4_f64 * t131584 - 0.11156198762715303246e-2_f64 * t124621 * t31993 * t3719 * t5236 + 0.18822977838986977999e-3_f64 * t124717 + 0.34271842599061411569e1_f64 * t8938 * t131592 * t131595 * t5465;
    (t131591, t131594, t131595, t131599)
}
