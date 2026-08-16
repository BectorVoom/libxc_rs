//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1384/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1384(t105780: f64, t105787: f64, t105797: f64, t105801: f64, t105810: f64, t105814: f64, t105818: f64, t105822: f64, t1408: f64, t1877: f64, t1915: f64, t23295: f64, t25013: f64, t2522: f64, t25358: f64, t28252: f64, t28256: f64, t28448: f64, t28456: f64, t28459: f64, t28462: f64, t4314: f64, t6670: f64, t7475: f64, t7541: f64, t82312: f64, t87975: f64) -> f64 {
    let t105829 = -3.0_f64 / 2.0_f64 * t1877 * t25358 * t28462 - 3.0_f64 / 2.0_f64 * t1877 * t6670 * t105780 + 9.0_f64 / 2.0_f64 * t2522 * t28448 * t7475 - t1877 * t6670 * t105787 / 2.0_f64 + 3.0_f64 * t1877 * t87975 * t28456 + 9.0_f64 * t2522 * t7541 * t28252 + 3.0_f64 / 2.0_f64 * t2522 * t1915 * t105797 + 9.0_f64 * t25013 * t105801 - 3.0_f64 * t1877 * t25358 * t28459 + 9.0_f64 / 2.0_f64 * t2522 * t7541 * t28256 + 9.0_f64 * t4314 * t1915 * t105810 + 3.0_f64 * t1877 * t23295 * t105814 - 3.0_f64 / 2.0_f64 * t1877 * t6670 * t105818 - 3.0_f64 * t1877 * t82312 * t105822 + 3.0_f64 / 2.0_f64 * t1877 * t28448 * t1408;
    t105829
}
