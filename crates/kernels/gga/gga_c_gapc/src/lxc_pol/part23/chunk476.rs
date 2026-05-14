//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 476/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk476<F: Float>(t2957: F, t2958: F, t2913: F, t2916: F, t2926: F, t2930: F, t2934: F, t2939: F, t2943: F, t2946: F, t2949: F, t2955: F, t2874: F, t2876: F, t2882: F, t2887: F, t2892: F, t2895: F, t2897: F, t2900: F, t2904: F, t2907: F) -> (F,) {
    let t2959 = t2957 * t2958;
    let t2961 = -0.30368356656884499037e-4 * t2913 - 0.10122785552294833012e-4 * t2916 + 0.14762395597096631476e-5 * t2926 - 0.30368356656884499037e-4 * t2930 - 0.21724560703384400956e-4 * t2934 + 0.21724560703384400956e-4 * t2939 + 0.21724560703384400956e-5 * t2943 - 0.386262689306174649e-5 * t2946 - 0.21724560703384400956e-4 * t2949 - 0.63363302051537836122e-5 * t2955 + 0.21724560703384400956e-4 * t2959;
    let t2962 = -0.17379648562707520765e-2 * t2874 + 0.10427789137624512459e-2 * t2876 - 0.10427789137624512459e-2 * t2882 - 0.3475929712541504153e-4 * t2887 + 0.61802030288987943842e-4 * t2892 + 0.10427789137624512459e-2 * t2895 + 0.13758888445476787272e-3 * t2897 - 0.10427789137624512459e-2 * t2900 - 0.13159621217983282916e-3 * t2904 + 0.30368356656884499037e-4 * t2907 + t2961;
    (t2962,)
}
