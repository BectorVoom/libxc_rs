//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1320/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1320(t1014: f64, t27940: f64, t3245: f64, t8051: f64, t15573: f64, t2173: f64, t27918: f64, t26784: f64, t26823: f64, t27812: f64, t27895: f64, t27915: f64, t7696: f64, t8034: f64, t93143: f64, t93145: f64, t93606: f64, t93610: f64, t93628: f64, t95769: f64) -> (f64, f64, f64) {
    let t96270 = t1014 * t27940;
    let t96273 = t3245 * t8051;
    let t96281 = 0.46336805555555555556e-3_f64 * t2173 * t15573 * t27918;
    let t96286 = 0.30891203703703703704e-3_f64 * t93606 + 0.1621345679012345679e-1_f64 * t93143 + 0.23168402777777777778e-3_f64 * t93610 - 0.88437037037037037034e-2_f64 * t96270 - 0.11054629629629629629e-2_f64 * t93145 - 0.55273148148148148147e-3_f64 * t96273 + t93628 - 0.2782641015625e-3_f64 * t27895 * t26784 - 0.37069444444444444444e-2_f64 * t7696 * t27915 + t96281 + 0.69505208333333333333e-3_f64 * t26823 * t8034 - 0.37134344353515625e-4_f64 * t27812 * t95769;
    (t96270, t96273, t96286)
}
