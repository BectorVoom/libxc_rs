//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 942/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk942(t3: f64, t3966: f64, t1484: f64, t1530: f64, t16596: f64, t1877: f64, t1915: f64, t193: f64, t202: f64, t23290: f64, t23295: f64, t2522: f64, t25353: f64, t25358: f64, t25365: f64, t25374: f64, t4119: f64, t4255: f64, t4303: f64, t4314: f64, t6666: f64, t6670: f64, t7541: f64, t776: f64, t868: f64, t870: f64) -> (f64, f64) {
    let t25588 = t3 * t3966;
    let t25882 = t193 * t202 * t25353 * t870 + 3.0_f64 * t1484 * t2522 * t6666 - t1530 * t1877 * t23290 - 3.0_f64 * t16596 * t2522 * t6670 + 2.0_f64 * t1877 * t23295 * t25374 - t1877 * t25358 * t868 - t1877 * t4303 * t6670 + 3.0_f64 * t1915 * t2522 * t4119 + 6.0_f64 * t1915 * t4255 * t4314 - 3.0_f64 * t2522 * t25365 * t6670 + 3.0_f64 * t2522 * t7541 * t776;
    (t25588, t25882)
}
