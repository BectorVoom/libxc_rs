//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta420 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1607;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1608;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta420(t1164: f64, t18926: f64, t4869: f64, t4875: f64, t18711: f64, t300: f64, t3375: f64, t6084: f64, t1157: f64, t3411: f64, t6102: f64, t18682: f64, t18685: f64, t18688: f64, t18690: f64, t18692: f64, t18694: f64, t18696: f64, t18837: f64, t18839: f64, t18917: f64, t18920: f64, t18922: f64, t18924: f64, t18914: f64, t475: f64, t1214: f64, t248: f64, t3508: f64, t5011: f64, t4977: f64, t4582: f64, t11692: f64, t1174: f64, t1213: f64, t1227: f64, t15610: f64, t15642: f64, t15645: f64, t18393: f64, t18397: f64, t18401: f64, t18574: f64, t18577: f64, t18580: f64, t18584: f64, t18590: f64, t18594: f64, t3506: f64, t3577: f64, t488: f64, t4974: f64, t4989: f64, t5005: f64, t5024: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t18928, t18930, t18932, t18936, t18938, t18939) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1607(t1164, t18926, t4869, t4875, t18711, t300, t3375, t6084, t1157, t3411, t6102, t18682, t18685, t18688, t18690, t18692, t18694, t18696, t18837, t18839, t18917, t18920, t18922, t18924);
        let (t18940, t18951) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1608(t18914, t18939, t475, t1214, t248, t3508, t5011, t4977, t4582, t11692, t1174, t1213, t1227, t15610, t15642, t15645, t18393, t18397, t18401, t18574, t18577, t18580, t18584, t18590, t18594, t3506, t3577, t488, t4974, t4989, t5005, t5024);
    (t18928, t18930, t18932, t18936, t18938, t18940, t18951)
}
