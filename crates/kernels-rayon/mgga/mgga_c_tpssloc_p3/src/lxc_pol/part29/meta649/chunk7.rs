//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2165/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2165(t86752: f64, t86801: f64, t87952: f64, t88001: f64, t12606: f64, t3: f64, t12915: f64, t13487: f64, t13191: f64, t13471: f64, t1530: f64, t16596: f64, t1877: f64, t1915: f64, t193: f64, t202: f64, t22959: f64, t23290: f64, t2379: f64, t25013: f64, t2522: f64, t25358: f64, t25365: f64, t25374: f64, t2553: f64, t4119: f64, t4314: f64, t57893: f64, t57912: f64, t6666: f64, t6670: f64, t7541: f64, t81525: f64, t81539: f64, t82312: f64, t86717: f64, t868: f64, t86836: f64, t870: f64, t87944: f64) -> (f64, f64, f64) {
    let t88003 = t86752 + t86801 + t87952 + t88001;
    let t88391 = t3 * t12606;
    let t89733 = t12915 * t13487;
    let t89775 = -2.0_f64 * t1877 * t86836 * t868 + 12.0_f64 * t22959 * t89733 + 4.0_f64 * t1877 * t81539 * t25374 - 6.0_f64 * t2522 * t23290 * t16596 - 12.0_f64 * t25013 * t57912 - 6.0_f64 * t2522 * t6670 * t57893 + 3.0_f64 * t2522 * t7541 * t2553 - 6.0_f64 * t2522 * t25358 * t13487 + 6.0_f64 * t2522 * t6666 * t4119 - 6.0_f64 * t2522 * t23290 * t25365 - 6.0_f64 * t1877 * t82312 * t86717 + 12.0_f64 * t4314 * t1915 * t13191 - t1877 * t6670 * t13471 + 6.0_f64 * t4314 * t7541 * t2379 - t1877 * t81525 * t1530 + t193 * t202 * t87944 * t870;
    (t88003, t88391, t89775)
}
