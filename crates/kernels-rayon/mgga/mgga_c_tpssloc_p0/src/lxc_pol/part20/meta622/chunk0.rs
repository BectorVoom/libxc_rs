//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2238/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2238(t12939: f64, t13126: f64, t2244: f64, t2745: f64, t868: f64, t16693: f64, t9682: f64, t1409: f64, t707: f64, t9862: f64, t13123: f64, t9467: f64) -> (f64, f64, f64, f64, f64) {
    let t46361 = 72.0_f64 * t12939 * t13126 * t2244;
    let t46362 = t2745 * t868;
    let t46367 = 36.0_f64 * t16693 * t9682;
    let t46369 = t707 * t9862 * t1409;
    let t46370 = 4.0_f64 * t46369;
    let t46371 = t13123 * t9467;
    (t46361, t46362, t46367, t46370, t46371)
}
