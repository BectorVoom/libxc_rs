//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1208/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1208(t10496: f64, t17245: f64, t3396: f64, t637: f64, t3401: f64, t5165: f64, t1535: f64, t1536: f64, t16701: f64, t16873: f64, t2536: f64, t2537: f64, t2706: f64, t2718: f64, t29093: f64, t29119: f64, t29120: f64, t29122: f64, t29124: f64, t568: f64, t8751: f64) -> (f64, f64) {
    let t29666 = t10496 * t17245;
    let t29677 = t3396 * t637;
    let t29684 = t3401 * t637;
    let t29691 = t10496 * t5165;
    let t29695 = -9.0_f64 * t1535 * t2537 * t29677 + 6.0_f64 * t1535 * t29691 * t568 + 18.0_f64 * t1536 * t2718 * t29093 - 3.0_f64 * t2536 * t2706 * t8751 - 18.0_f64 * t2537 * t2718 * t29684 + t16701 + t16873 - t29119 + t29120 + t29122 - t29124;
    (t29666, t29695)
}
