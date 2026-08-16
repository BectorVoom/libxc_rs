//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1147/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1147(t2096: f64, t2665: f64, t565: f64, t10711: f64, t11696: f64, t37936: f64, t10710: f64, t10728: f64, t24902: f64, t11699: f64, t37939: f64, t1592: f64, t25172: f64, t3308: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39960 = t2665 * t2096;
    let t39961 = t565 * t39960;
    let t39962 = t39961 * t10711;
    let t39964 = t37936 * t11696;
    let t39967 = t10728 * t10710 * t24902;
    let t39969 = t37939 * t11699;
    let t39972 = t1592 * t3308 * t25172;
    (t39960, t39962, t39964, t39967, t39969, t39972)
}
