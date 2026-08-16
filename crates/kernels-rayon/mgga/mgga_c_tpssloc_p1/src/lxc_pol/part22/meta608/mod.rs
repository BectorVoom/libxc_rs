//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta608 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2134;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2135;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta608(t10470: f64, t11058: f64, t381: f64, t1615: f64, t6739: f64, t11064: f64, t3199: f64, t49649: f64, t11045: f64, t10164: f64, t1634: f64, t11190: f64, t1670: f64, t3242: f64, t457: f64, t2394: f64, t4734: f64, t1654: f64, t9698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50508, t50509, t50516, t50592, t50610, t50628, t50819) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2134(t10470, t11058, t381, t1615, t6739, t11064, t3199, t49649, t11045, t10164, t1634, t11190, t1670);
        let (t50822, t50826, t50827, t50834) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2135(t3242, t457, t2394, t4734, t1654, t9698);
    (t50508, t50509, t50516, t50592, t50610, t50628, t50819, t50822, t50826, t50827, t50834)
}
