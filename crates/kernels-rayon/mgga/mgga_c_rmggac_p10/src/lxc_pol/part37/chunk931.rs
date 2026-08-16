//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 931/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk931(t76840: f64, t74050: f64, t74052: f64, t74056: f64, t74058: f64, t74063: f64, t74065: f64, t74082: f64, t74084: f64, t74088: f64, t74092: f64, t74096: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t76841 = 0.40650199722100037752e-3_f64 * t76840;
    let t76842 = 0.20455996240684006296e-1_f64 * t74050;
    let t76843 = 0.81823984962736025184e-1_f64 * t74052;
    let t76844 = 0.20455996240684006296e0_f64 * t74056;
    let t76846 = 0.40911992481368012592e-1_f64 * t74058;
    let t76848 = 0.2627895913935205078e-5_f64 * t74063;
    let t76849 = 0.2627895913935205078e-5_f64 * t74065;
    let t76854 = 0.35913881159970051994e-4_f64 * t74082;
    let t76855 = 0.3830813990396805546e-4_f64 * t74084;
    let t76856 = 0.2553875993597870364e-4_f64 * t74088;
    let t76857 = 0.2553875993597870364e-4_f64 * t74092;
    let t76858 = 0.1702583995731913576e-4_f64 * t74096;
    (t76841, t76842, t76843, t76844, t76846, t76848, t76849, t76854, t76855, t76856, t76857, t76858)
}
