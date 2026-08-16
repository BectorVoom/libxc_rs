//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2338/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2338(t676: f64, t9387: f64, t2629: f64, t9372: f64, t2434: f64, t2516: f64, t8779: f64, t9645: f64, t252: f64, t685: f64, t788: f64, t10115: f64, t862: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39532 = t676 * t9387;
    let t39534 = 0.21687162600603479684e-1_f64 * t2629 * t39532;
    let t39535 = t676 * t9372;
    let t39537 = 0.38025319932552508021e2_f64 * t2629 * t39535;
    let t39538 = t2434 * t2516;
    let t39540 = 0.43374325201206959368e-1_f64 * t2629 * t39538;
    let t39545 = t8779 * t9645;
    let t39549 = 0.65457331274007190912e-5_f64 * t39545 * t252 * t788 * t685;
    let t39550 = t10115 * t862;
    (t39532, t39534, t39535, t39537, t39538, t39540, t39545, t39549, t39550)
}
