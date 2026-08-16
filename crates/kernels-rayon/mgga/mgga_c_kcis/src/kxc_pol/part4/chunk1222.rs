//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1222/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1222(t11220: f64, t1282: f64, t1291: f64, t14664: f64, t14672: f64, t14674: f64, t14676: f64, t14679: f64, t15093: f64, t15109: f64, t15690: f64, t15692: f64, t15788: f64, t1872: f64, t3670: f64, t437: f64) -> f64 {
    let t15790 = -t11220 * t1872 - t1282 * t15788 - 2.0_f64 * t1291 * t15109 + t15690 * t437 + 2.0_f64 * t15692 * t3670 - t14664 + t14672 - t14674 + t14676 + t14679 + t15093;
    t15790
}
