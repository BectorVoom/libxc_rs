//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1417/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1417(t1227: f64, t15743: f64, t1725: f64, t698: f64, t1174: f64, t225: f64, t4941: f64, t5053: f64, t3701: f64, t5356: f64, t5168: f64, t592: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15745 = 5.0_f64 / 10368.0_f64 * t1227 * t15743;
    let t15753 = t698 * t1725;
    let t15754 = t1174 * t15753;
    let t15797 = t4941 * t225;
    let t15820 = t5053 * t225;
    let t15868 = t5356 * t3701;
    let t15877 = t592 * t5168;
    (t15745, t15754, t15797, t15820, t15868, t15877)
}
