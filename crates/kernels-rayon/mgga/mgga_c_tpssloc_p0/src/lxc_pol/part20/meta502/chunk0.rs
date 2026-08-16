//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2011/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2011(t3014: f64, t343: f64, t3469: f64, t460: f64, t3475: f64, t253: f64, t254: f64, t1519: f64, t828: f64, t382: f64, t1453: f64, t666: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23547 = t3014 * t343;
    let t24698 = t3469 * t460;
    let t24705 = t3475 * t460;
    let t25168 = t253 * t254;
    let t25236 = t1519 * t828;
    let t25757 = t382 * t254;
    let t26129 = t1453 * t666;
    (t23547, t24698, t24705, t25168, t25236, t25757, t26129)
}
