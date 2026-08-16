//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1220/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1220(t345: f64, t6805: f64, t1022: f64, t1945: f64, t1060: f64, t383: f64, t6768: f64, t1003: f64, t1058: f64, t1920: f64, t1950: f64, t1953: f64, t353: f64, t6680: f64, t6687: f64, t6783: f64, t6787: f64, t6790: f64, t6797: f64, t6802: f64) -> (f64, f64, f64, f64) {
    let t6806 = t345 * t6805;
    let t6810 = t1945 * t1022;
    let t6811 = t6810 * t1060;
    let t6813 = t383 * t6768;
    let t6815 = -0.21932454224643019153e-1_f64 * t6680 * t1950 + t6783 + 0.27415567780803773942e-2_f64 * t6687 * t6787 - 0.82246703342411321825e-2_f64 * t6687 * t6790 + 0.82246703342411321825e-2_f64 * t6797 * t6802 + 0.82246703342411321825e-2_f64 * t1920 * t6806 + t1003 * t1953 + t1058 * t6811 + t353 * t6813;
    (t6806, t6811, t6813, t6815)
}
