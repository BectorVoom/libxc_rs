//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2037/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2037(t2098: f64, t5381: f64, t27286: f64, t576: f64, t112: f64, t27240: f64, t12521: f64, t12524: f64, t1401: f64, t1458: f64, t16521: f64, t16524: f64, t2039: f64, t2363: f64, t23917: f64, t24462: f64, t24478: f64, t24481: f64, t27170: f64, t27254: f64, t27273: f64, t27276: f64, t3941: f64, t4072: f64, t5371: f64, t5376: f64, t55353: f64, t55405: f64, t671: f64, t7056: f64, t7235: f64, t7801: f64, t84033: f64, t84078: f64, t92128: f64) -> (f64, f64, f64) {
    let t94120 = 2.0_f64 * t2098 * t5381;
    let t94122 = 2.0_f64 * t576 * t27286;
    let t94127 = t27240 * t112;
    let t94160 = 0.135e2_f64 * t84078 * t1458 + 27.0_f64 * t55405 * t2039 + 27.0_f64 * t94127 * t671 + 54.0_f64 * t12524 * t27273 + 54.0_f64 * t12524 * t27276 + 27.0_f64 * t24462 * t4072 + 0.135e2_f64 * t27254 * t2363 + 0.135e2_f64 * t12521 * t7801 + 54.0_f64 * t84033 * t5376 + 54.0_f64 * t3941 * t27170 * t671 + 27.0_f64 * t3941 * t7801 * t2363 + 54.0_f64 * t55353 * t7235 + 54.0_f64 * t16524 * t24478 + 27.0_f64 * t16521 * t7056 + 0.135e2_f64 * t5371 * t23917 + 0.135e2_f64 * t1401 * t92128 + 27.0_f64 * t16524 * t24481;
    (t94120, t94122, t94160)
}
