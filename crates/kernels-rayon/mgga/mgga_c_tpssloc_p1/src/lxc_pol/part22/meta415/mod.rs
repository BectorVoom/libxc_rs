//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta415 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1718;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1719;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1720;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1721;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta415(t1100: f64, t18730: f64, t1107: f64, t11243: f64, t5992: f64, t1102: f64, t4756: f64, t4764: f64, t3287: f64, t5999: f64, t11265: f64, t4748: f64, t11211: f64, t11372: f64, t14702: f64, t14705: f64, t14711: f64, t3270: f64, t11137: f64, t14818: f64, t18227: f64, t18239: f64, t18497: f64, t18500: f64, t18503: f64, t18508: f64, t18510: f64, t18515: f64, t18518: f64, t11369: f64, t14722: f64, t14766: f64, t14768: f64, t14782: f64, t18203: f64, t18208: f64, t18213: f64, t18217: f64, t18219: f64, t18223: f64, t18229: f64, t18234: f64, t18243: f64, t18494: f64, t18505: f64, t18512: f64, t18521: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18731, t18742, t18746, t18747, t18749, t18752, t18754, t18755, t18757) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1718(t1100, t18730, t1107, t11243, t5992, t1102, t4756, t4764, t3287, t5999, t11265, t4748);
        let (t18759, t18761) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1719(t11211, t11372, t14702, t14705, t14711, t18742, t18747, t18749, t18752, t18755, t18757, t3270, t5999);
        let (t18762, t18783) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1720(t1102, t18761, t11137, t14818, t18227, t18239, t18497, t18500, t18503, t18508, t18510, t18515, t18518);
        let t18785 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1721(t11369, t14722, t14766, t14768, t14782, t18203, t18208, t18213, t18217, t18219, t18223, t18229, t18234, t18243, t18494, t18505, t18512, t18521, t18731, t18759, t18762, t18783);
    (t18731, t18742, t18746, t18747, t18749, t18752, t18754, t18755, t18757, t18762, t18785)
}
