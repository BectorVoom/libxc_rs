//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 988/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk988(t39060: f64, t7785: f64, t39880: f64, t39064: f64, t7788: f64, t2347: f64, t866: f64, t262: f64, t2350: f64, t876: f64, t36274: f64, t38569: f64, t7782: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40877 = t7785 * t39060;
    let t40879 = t7785 * t39880;
    let t40881 = t7788 * t39064;
    let t40883 = t2347 * t866;
    let t40884 = t262 * t40883;
    let t40885 = t7788 * t40884;
    let t40887 = t2350 * t876;
    let t40888 = t262 * t40887;
    let t40889 = t36274 * t40888;
    let t40891 = t7782 * t38569;
    (t40877, t40879, t40881, t40883, t40884, t40885, t40887, t40888, t40889, t40891)
}
