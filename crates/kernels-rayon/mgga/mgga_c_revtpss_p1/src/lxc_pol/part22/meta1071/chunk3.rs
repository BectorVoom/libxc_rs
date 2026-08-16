//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3839/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3839(t22299: f64, t9962: f64, t22295: f64, t22111: f64, t22115: f64, t13999: f64, t22163: f64, t22048: f64, t22089: f64, t13789: f64, t13926: f64, t22046: f64, t22096: f64, t3934: f64, t3936: f64, t46592: f64, t48102: f64, t9810: f64) -> f64 {
    let t73798 = t9962 * t22299;
    let t73800 = t9962 * t22295;
    let t73803 = t9962 * t22111;
    let t73805 = t9962 * t22115;
    let t73811 = t13999 * t22163;
    let t73813 = t13999 * t22048;
    let t73815 = t13999 * t22089;
    let t73817 = 0.34299214494455789578e-2_f64 * t3934 * t13789 * t13926 * t22096 - 0.4065600224742826258e-3_f64 * t48102 - 0.16006300097412701803e-1_f64 * t73798 + 0.80031500487063509016e-1_f64 * t73800 - 0.50820002809285328225e-4_f64 * t46592 + 0.40015750243531754508e-2_f64 * t73803 + 0.20007875121765877254e-2_f64 * t73805 + 0.85748036236139473944e-3_f64 * t3934 * t3936 * t22046 * t9810 - 0.12004725073059526352e-1_f64 * t73811 + 0.16006300097412701803e-1_f64 * t73813 - 0.80031500487063509015e-2_f64 * t73815;
    t73817
}
