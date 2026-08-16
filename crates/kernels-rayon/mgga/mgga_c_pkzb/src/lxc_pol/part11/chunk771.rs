//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 771/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk771(t2595: f64, t6892: f64, t168: f64, t5389: f64, t2591: f64, t1034: f64, t5391: f64, t2583: f64, t5221: f64, t1702: f64, t2587: f64, t1025: f64, t5264: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6894 = 0.40015750243531754508e-2_f64 * t6892 * t2595;
    let t6895 = t5389 * t168;
    let t6896 = t6895 * t2591;
    let t6897 = t1034 * t5391;
    let t6914 = 7.0_f64 / 24.0_f64 * t5221 * t2583;
    let t6928 = 7.0_f64 / 72.0_f64 * t1702 * t2587;
    let t6933 = t5264 * t1025;
    (t6894, t6895, t6896, t6897, t6914, t6928, t6933)
}
