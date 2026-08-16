//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1076/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1076(t191: f64, t192: f64, t6872: f64, t2020: f64, t6876: f64, t8494: f64, t6997: f64, t8450: f64, t1393: f64, t31062: f64, t31065: f64, t31067: f64, t31070: f64, t31072: f64, t31077: f64, t31078: f64, t31080: f64, t31082: f64, t31088: f64, t31089: f64, t31223: f64, t31224: f64, t31240: f64, t574: f64, t6517: f64, t652: f64, t6539: f64, t672: f64, t8447: f64) -> (f64, f64) {
    let t31246 = t6872 * t191 * t192;
    let t31247 = t31246 * t2020;
    let t31249 = t6876 * t8494;
    let t31250 = t8450 * t6997;
    let t31252 = t1393 * t8447 - 2.0_f64 * t31062 * t652 - 2.0_f64 * t31224 * t672 + t31240 * t574 - 4.0_f64 * t6517 * t6539 - 4.0_f64 * t31065 - 4.0_f64 * t31067 - 4.0_f64 * t31070 - 4.0_f64 * t31072 - t31077 - 4.0_f64 * t31078 - 4.0_f64 * t31080 - 4.0_f64 * t31082 - t31088 + t31089 + t31223 + 2.0_f64 * t31247 - t31249 + 2.0_f64 * t31250;
    (t31246, t31252)
}
