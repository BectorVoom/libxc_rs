//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 513/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk513(t1343: f64, t6388: f64, t820: f64, t1799: f64, t550: f64, t3805: f64, t5249: f64, t5264: f64, t5266: f64, t2408: f64, t2417: f64, t2426: f64, t2486: f64, t3688: f64, t3813: f64, t6299: f64, t6304: f64, t6329: f64) -> (f64, f64, f64, f64, f64) {
    let t6390 = t1343 * t820 * t6388;
    let t6394 = t550 * t1799;
    let t6396 = t3805 * t5249 * t6394;
    let t6399 = 8.0_f64 * t5264;
    let t6400 = 8.0_f64 * t5266;
    let t6401 = t6329 + t6304 + t3813 - t2486 - t6299 + t2408 + t2417 - t6399 - t6400 - t2426 + t3688;
    (t6390, t6396, t6399, t6400, t6401)
}
