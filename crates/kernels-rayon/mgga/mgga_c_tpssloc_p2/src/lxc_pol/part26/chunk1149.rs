//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1149/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1149(t134: f64, t221: f64, t2250: f64, t3: f64, t3034: f64, t371: f64, t13487: f64, t1877: f64, t1915: f64, t193: f64, t202: f64, t23285: f64, t23290: f64, t23295: f64, t2379: f64, t2522: f64, t2553: f64, t2745: f64, t2749: f64, t4314: f64, t6666: f64, t6670: f64, t776: f64, t868: f64, t870: f64) -> (f64, f64, f64, f64, f64) {
    let t23383 = t221 * t134;
    let t23413 = t3 * t2250;
    let t23508 = 1.0_f64 / t3034 / t371;
    let t23598 = 1.0_f64 / t3034;
    let t23772 = t193 * t202 * t23285 * t870 - 6.0_f64 * t13487 * t2522 * t6670 - 2.0_f64 * t1877 * t23290 * t868 + 2.0_f64 * t1877 * t23295 * t2749 - t1877 * t2745 * t6670 + 6.0_f64 * t1915 * t2379 * t4314 + 3.0_f64 * t1915 * t2522 * t2553 + 6.0_f64 * t2522 * t6666 * t776;
    (t23383, t23413, t23508, t23598, t23772)
}
