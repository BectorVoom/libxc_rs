//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2174/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2174(t11923: f64, t225: f64, t10913: f64, t11583: f64, t11570: f64, t1174: f64, t3471: f64, t698: f64, t3477: f64, t11504: f64, t135: f64, t43776: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44412 = t11923 * t225;
    let t44415 = t11583 * t10913;
    let t44419 = t11570 * t10913;
    let t44424 = t1174 * t698 * t3471;
    let t44439 = t1174 * t698 * t3477;
    let t44445 = t1174 * t135 * t11504;
    let t44466 = 220.0_f64 / 81.0_f64 * t43776;
    (t44412, t44415, t44419, t44424, t44439, t44445, t44466)
}
