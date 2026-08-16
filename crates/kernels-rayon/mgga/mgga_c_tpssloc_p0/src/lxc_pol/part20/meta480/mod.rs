//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta480 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1961;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1962;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta480(t15968: f64, t182: f64, t1787: f64, t2516: f64, t17: f64, t12097: f64, t12100: f64, t12111: f64, t12120: f64, t184: f64, t2663: f64, t5157: f64, t12103: f64, t12105: f64, t12109: f64, t12114: f64, t12116: f64, t12118: f64, t12123: f64, t12477: f64, t1799: f64, t3719: f64, t3918: f64, t5122: f64, t9797: f64, t9820: f64, t9824: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15970, t15971, t15972, t15973, t15974, t15975, t15976, t15977, t15978, t15979) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1961(t15968, t182, t1787, t2516, t17, t12097, t12100, t12111, t12120, t184, t2663, t5157);
        let (t15980, t15981) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1962(t15979, t12103, t12105, t12109, t12114, t12116, t12118, t12123, t12477, t15970, t15972, t15973, t15974, t15975, t15976, t15978, t1799, t3719, t3918, t5122, t9797, t9820, t9824);
    (t15970, t15971, t15972, t15973, t15974, t15975, t15976, t15977, t15978, t15980, t15981)
}
