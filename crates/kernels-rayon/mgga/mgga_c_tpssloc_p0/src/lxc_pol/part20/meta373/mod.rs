//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta373 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1721;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1722;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta373(t1484: f64, t212: f64, t9523: f64, t2586: f64, t213: f64, t4119: f64, t221: f64, t776: f64, t2553: f64, t4128: f64, t2570: f64, t67: f64, t792: f64, t686: f64, t4127: f64, t9526: f64, t9540: f64, t9542: f64, t9544: f64, t9547: f64, t9552: f64, t9556: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12984, t12985, t12986, t12988, t12990, t12994, t12997) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1721(t1484, t212, t9523, t2586, t213, t4119, t221, t776, t2553, t4128, t2570, t67);
        let (t12998, t13000, t13003) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1722(t12997, t792, t12984, t686, t776, t12986, t12990, t12994, t4127, t9526, t9540, t9542, t9544, t9547, t9552, t9556);
    (t12984, t12985, t12988, t12990, t12994, t12997, t12998, t13000, t13003)
}
