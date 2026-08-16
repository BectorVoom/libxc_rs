//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 505/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk505(t2874: f64, t2876: f64, t2882: f64, t2887: f64, t2892: f64, t2895: f64, t2897: f64, t2900: f64, t2904: f64, t2907: f64, t2961: f64, t1010: f64, t575: f64) -> (f64, f64) {
    let t2962 = -0.17379648562707520765e-2_f64 * t2874 + 0.10427789137624512459e-2_f64 * t2876 - 0.10427789137624512459e-2_f64 * t2882 - 0.3475929712541504153e-4_f64 * t2887 + 0.61802030288987943842e-4_f64 * t2892 + 0.10427789137624512459e-2_f64 * t2895 + 0.13758888445476787272e-3_f64 * t2897 - 0.10427789137624512459e-2_f64 * t2900 - 0.13159621217983282916e-3_f64 * t2904 + 0.30368356656884499037e-4_f64 * t2907 + t2961;
    let t2964 = t1010 * t575;
    (t2962, t2964)
}
