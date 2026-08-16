//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1513/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1513(t1307: f64, t1388: f64, t118: f64, t1787: f64, t2375: f64, t12045: f64, t12050: f64, t12052: f64, t12054: f64, t5151: f64, t750: f64, t17: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15904 = t1388 * t1307;
    let t15908 = t1787 * t118;
    let t15909 = t15908 * t2375;
    let t15910 = 0.10843581300301739842e-1_f64 * t15909;
    let t15911 = 48.0_f64 * t12045;
    let t15915 = 24.0_f64 * t12050;
    let t15916 = 12.0_f64 * t12052;
    let t15917 = 80.0_f64 * t12054;
    let t15921 = t5151 * t750;
    let t15923 = 2.0_f64 * t17 * t15921;
    (t15904, t15910, t15911, t15915, t15916, t15917, t15923)
}
