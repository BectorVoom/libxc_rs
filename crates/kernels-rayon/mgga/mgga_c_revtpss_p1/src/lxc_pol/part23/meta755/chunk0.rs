//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2545/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2545(t53542: f64, t3115: f64, t42793: f64, t4906: f64, t3162: f64, t999: f64, t42865: f64, t72: f64, t3088: f64, t43472: f64, t43401: f64, t1062: f64, t15655: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t53543 = t53542 / 432.0_f64;
    let t53612 = t3115 * t42793 * t4906;
    let t53613 = 0.14291339372689912324e-3_f64 * t53612;
    let t53619 = t3162 * t999;
    let t53667 = t42865 * t72;
    let t53668 = t3088 * t53667;
    let t53669 = t43472 * t53668;
    let t53676 = t43401 * t53668;
    let t53692 = t15655 * t1062;
    (t53543, t53613, t53619, t53667, t53668, t53669, t53676, t53692)
}
