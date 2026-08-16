//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta414 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1716;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1717;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta414(t1119: f64, t18686: f64, t14845: f64, t1671: f64, t4740: f64, t4782: f64, t11424: f64, t5989: f64, t3259: f64, t6021: f64, t11136: f64, t11137: f64, t14702: f64, t14922: f64, t14923: f64, t14924: f64, t18203: f64, t18208: f64, t18213: f64, t18217: f64, t18219: f64, t18223: f64, t18227: f64, t18229: f64, t18234: f64, t18239: f64, t18243: f64, t449: f64, t11247: f64, t14721: f64, t14723: f64, t14724: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18688, t18690, t18692, t18694, t18696, t18710) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1716(t1119, t18686, t14845, t1671, t4740, t4782, t11424, t5989, t3259, t6021, t11136, t11137, t14702, t14922, t14923, t14924, t18203, t18208, t18213, t18217, t18219, t18223, t18227, t18229, t18234, t18239, t18243);
        let (t18711, t18730) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1717(t18710, t449, t11137, t11247, t14702, t14721, t14723, t14724, t18203, t18208, t18213, t18217, t18219, t18223, t18227, t18229, t18234, t18239, t18243);
    (t18688, t18690, t18692, t18694, t18696, t18710, t18711, t18730)
}
