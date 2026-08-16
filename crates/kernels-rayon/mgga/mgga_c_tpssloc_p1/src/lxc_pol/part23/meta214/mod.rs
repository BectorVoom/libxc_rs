//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta214 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk858;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk859;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta214(t2628: f64, t836: f64, t812: f64, t242: f64, t9972: f64, t2638: f64, t4166: f64, t2629: f64, t820: f64, t9645: f64, t2696: f64, t1516: f64, t9601: f64, t68: f64, t9971: f64, t226: f64, t1519: f64, t2627: f64, t1543: f64, t2841: f64, t1540: f64, t2394: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13258, t13262, t13278, t13283, t13350, t13360, t13368) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk858(t2628, t836, t812, t242, t9972, t2638, t4166, t2629, t820, t9645, t2696, t1516, t9601);
        let (t13397, t13416, t13520, t13598) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk859(t68, t9971, t226, t1519, t2627, t1543, t2841, t1540, t2394);
    (t13258, t13262, t13278, t13283, t13350, t13360, t13368, t13397, t13416, t13520, t13598)
}
