//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1215/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1215(t24487: f64, t2508: f64, t948: f64, t2586: f64, t8637: f64, t29277: f64, t7064: f64, t8970: f64, t3431: f64, t701: f64, t2610: f64, t10752: f64, t5288: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32253 = 0.23071578690426672851e-1_f64 * t2508 * t24487 * t948;
    let t32256 = 0.46143157380853345702e-1_f64 * t2508 * t8637 * t2586;
    let t32258 = t7064 * t29277 * t8970;
    let t32259 = 0.1281754371690370714e-2_f64 * t32258;
    let t32260 = t3431 * t701;
    let t32261 = t2610 * t32260;
    let t32266 = 0.46143157380853345702e-1_f64 * t5288 * t10752;
    (t32253, t32256, t32259, t32260, t32261, t32266)
}
