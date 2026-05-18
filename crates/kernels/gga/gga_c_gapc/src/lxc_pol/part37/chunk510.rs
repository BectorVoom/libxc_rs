//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 510/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk510<F: Float>(t2953: F, t2954: F, t1005: F, t1599: F, t1603: F, t2937: F, t2913: F, t2916: F, t2926: F, t2930: F, t2934: F, t2939: F, t2943: F, t2946: F, t2949: F) -> (F, F, F, F, F) {
    let t2955 = t2953 * t2954;
    let t2957 = t1005 * t1599;
    let t2958 = t2937 * t1603;
    let t2959 = t2957 * t2958;
    let t2961 = -F::new(0.30368356656884499037e-4) * t2913 - F::new(0.10122785552294833012e-4) * t2916 + F::new(0.14762395597096631476e-5) * t2926 - F::new(0.30368356656884499037e-4) * t2930 - F::new(0.21724560703384400956e-4) * t2934 + F::new(0.21724560703384400956e-4) * t2939 + F::new(0.21724560703384400956e-5) * t2943 - F::new(0.386262689306174649e-5) * t2946 - F::new(0.21724560703384400956e-4) * t2949 - F::new(0.63363302051537836122e-5) * t2955 + F::new(0.21724560703384400956e-4) * t2959;
    (t2955, t2957, t2958, t2959, t2961)
}
