//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3839/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3839<F: Float>(t22299: F, t9962: F, t22295: F, t22111: F, t22115: F, t13999: F, t22163: F, t22048: F, t22089: F, t13789: F, t13926: F, t22046: F, t22096: F, t3934: F, t3936: F, t46592: F, t48102: F, t9810: F) -> F {
    let t73798 = t9962 * t22299;
    let t73800 = t9962 * t22295;
    let t73803 = t9962 * t22111;
    let t73805 = t9962 * t22115;
    let t73811 = t13999 * t22163;
    let t73813 = t13999 * t22048;
    let t73815 = t13999 * t22089;
    let t73817 = F::cast_from(0.34299214494455789578e-2_f64) * t3934 * t13789 * t13926 * t22096 - F::cast_from(0.4065600224742826258e-3_f64) * t48102 - F::cast_from(0.16006300097412701803e-1_f64) * t73798 + F::cast_from(0.80031500487063509016e-1_f64) * t73800 - F::cast_from(0.50820002809285328225e-4_f64) * t46592 + F::cast_from(0.40015750243531754508e-2_f64) * t73803 + F::cast_from(0.20007875121765877254e-2_f64) * t73805 + F::cast_from(0.85748036236139473944e-3_f64) * t3934 * t3936 * t22046 * t9810 - F::cast_from(0.12004725073059526352e-1_f64) * t73811 + F::cast_from(0.16006300097412701803e-1_f64) * t73813 - F::cast_from(0.80031500487063509015e-2_f64) * t73815;
    t73817
}
