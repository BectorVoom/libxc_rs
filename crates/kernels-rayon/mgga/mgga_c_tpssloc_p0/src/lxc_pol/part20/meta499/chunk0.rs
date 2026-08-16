//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2007/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2007(t1484: f64, t868: f64, t13115: f64, t157: f64, t1504: f64, t68: f64, t1499: f64, t4290: f64, t4166: f64, t4177: f64, t2632: f64, t4233: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16596 = t1484 * t868;
    let t16693 = t13115 * t157;
    let t16729 = t1504 * t68;
    let t16830 = t1499 * t4290;
    let t16836 = t4166 * t4177;
    let t16935 = t2632 * t4233;
    (t16596, t16693, t16729, t16830, t16836, t16935)
}
