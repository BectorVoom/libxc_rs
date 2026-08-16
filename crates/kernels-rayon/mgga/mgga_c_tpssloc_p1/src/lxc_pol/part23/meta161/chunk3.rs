//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 752/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk752(t5264: f64, t5266: f64, t2408: f64, t2417: f64, t2426: f64, t2486: f64, t3688: f64, t3813: f64, t6299: f64, t6304: f64, t6329: f64, t2423: f64, t3686: f64, t3690: f64, t3695: f64, t3819: f64, t3821: f64, t3823: f64, t3825: f64, t3832: f64, t3836: f64, t6300: f64, t6322: f64) -> (f64, f64, f64, f64) {
    let t6399 = 8.0_f64 * t5264;
    let t6400 = 8.0_f64 * t5266;
    let t6401 = t6329 + t6304 + t3813 - t2486 - t6299 + t2408 + t2417 - t6399 - t6400 - t2426 + t3688;
    let t6402 = -t3690 - t3695 + t6322 + t3686 + t3819 + t3821 + t3823 - t2423 - t6300 + t3825 - t3832 - t3836;
    (t6399, t6400, t6401, t6402)
}
