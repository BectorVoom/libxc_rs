//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1261/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1261(t26796: f64, t303: f64, t6614: f64, t1092: f64, t27788: f64, t95664: f64, t15573: f64, t29151: f64, t7788: f64, t18502: f64, t7726: f64, t1749: f64, t5013: f64) -> (f64, f64, f64, f64, f64) {
    let t100619 = t303 * t26796 * t6614;
    let t100622 = t1092 * t95664 * t27788;
    let t100629 = t7788 * t15573 * t29151;
    let t100636 = t303 * t7726 * t18502;
    let t100641 = t303 * t1749 * t5013;
    (t100619, t100622, t100629, t100636, t100641)
}
