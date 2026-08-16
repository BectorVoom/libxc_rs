//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 949/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk949(t74058: f64, t74063: f64, t74065: f64, t74082: f64, t74084: f64, t74088: f64, t74092: f64, t74096: f64, t74102: f64, t70819: f64, t74060: f64, t74069: f64, t74072: f64, t74075: f64, t74078: f64, t74105: f64, t74107: f64) -> f64 {
    let t76846 = 0.40911992481368012592e-1_f64 * t74058;
    let t76848 = 0.2627895913935205078e-5_f64 * t74063;
    let t76849 = 0.2627895913935205078e-5_f64 * t74065;
    let t76854 = 0.35913881159970051994e-4_f64 * t74082;
    let t76855 = 0.3830813990396805546e-4_f64 * t74084;
    let t76856 = 0.2553875993597870364e-4_f64 * t74088;
    let t76857 = 0.2553875993597870364e-4_f64 * t74092;
    let t76858 = 0.1702583995731913576e-4_f64 * t74096;
    let t76859 = 0.2553875993597870364e-4_f64 * t74102;
    let t76862 = -t76846 + t70819 + 0.17451485956252114154e-4_f64 * t74060 + t76848 - t76849 + 0.17519306092901367187e-5_f64 * t74069 + 0.52557918278704101564e-6_f64 * t74072 - 0.52557918278704101564e-6_f64 * t74075 - 0.35038612185802734376e-6_f64 * t74078 - t76854 + t76855 + t76856 - t76857 - t76858 + t76859 + 0.58171619854173713846e-5_f64 * t74105 + 0.36357262408858571154e-4_f64 * t74107;
    t76862
}
