//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta552 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2245;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2246;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta552(t1136: f64, t6037: f64, t1683: f64, t4819: f64, t6056: f64, t6053: f64, t3359: f64, t6052: f64, t4823: f64, t11352: f64, t6036: f64, t11137: f64, t11444: f64, t14702: f64, t14720: f64, t15194: f64, t15195: f64, t18203: f64, t18208: f64, t18213: f64, t18217: f64, t18219: f64, t18223: f64, t18227: f64, t18229: f64, t18234: f64, t18239: f64, t18243: f64, t14838: f64, t4745: f64, t11350: f64, t11420: f64, t18257: f64, t18261: f64, t18264: f64, t18268: f64, t3332: f64, t3357: f64, t436: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18631, t18634, t18637, t18640, t18643, t18644, t18647, t18650, t18651, t18668) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2245(t1136, t6037, t1683, t4819, t6056, t6053, t3359, t6052, t4823, t11352, t6036, t11137, t11444, t14702, t14720, t15194, t15195, t18203, t18208, t18213, t18217, t18219, t18223, t18227, t18229, t18234, t18239, t18243);
        let (t18672, t18673) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2246(t14838, t4745, t11350, t11420, t18257, t18261, t18264, t18268, t18631, t18634, t18637, t18640, t18644, t18647, t18651, t18668, t3332, t3357, t436);
    (t18631, t18634, t18637, t18640, t18643, t18644, t18647, t18650, t18651, t18668, t18672, t18673)
}
