//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta458 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1665;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1666;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta458(t23956: f64, t24446: f64, t3: f64, t112: f64, t7222: f64, t111: f64, t2098: f64, t671: f64, t7056: f64, t2039: f64, t2363: f64, t12521: f64, t12524: f64, t1401: f64, t16535: f64, t2319: f64, t23917: f64, t3938: f64, t3941: f64, t577: f64, t7230: f64, t7235: f64, t191: f64, t192: f64, t5118: f64, t1390: f64, t5187: f64, t531: f64, t1982: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24447, t24448, t24462, t24465, t24478, t24481, t24486) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1665(t23956, t24446, t3, t112, t7222, t111, t2098, t671, t7056, t2039, t2363, t12521, t12524, t1401, t16535, t2319, t23917, t3938, t3941, t577, t7230, t7235);
        let (t24987, t24990, t24994, t24995) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1666(t191, t192, t5118, t1390, t5187, t531, t1982);
    (t24447, t24448, t24462, t24465, t24478, t24481, t24486, t24987, t24990, t24994, t24995)
}
