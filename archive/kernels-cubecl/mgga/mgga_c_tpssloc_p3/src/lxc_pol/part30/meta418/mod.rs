//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta418 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1594;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1595;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1596;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1597;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1598;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1599;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta418<F: Float>(t18710: F, t449: F, t11137: F, t11247: F, t14702: F, t14721: F, t14723: F, t14724: F, t18203: F, t18208: F, t18213: F, t18217: F, t18219: F, t18223: F, t18227: F, t18229: F, t18234: F, t18239: F, t18243: F, t1100: F, t1107: F, t11243: F, t5992: F, t1102: F, t4756: F, t4764: F, t3287: F, t5999: F, t11265: F, t4748: F, t11211: F, t11372: F, t14705: F, t14711: F, t3270: F, t14818: F, t18497: F, t18500: F, t18503: F, t18508: F, t18510: F, t18515: F, t18518: F, t11369: F, t14722: F, t14766: F, t14768: F, t14782: F, t18494: F, t18505: F, t18512: F, t18521: F, t1156: F, t11297: F, t1148: F, t18676: F, t18679: F, t18682: F, t18685: F, t18688: F, t18690: F, t18692: F, t18694: F, t18696: F, t3371: F, t6069: F, t6085: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t18711, t18730) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1594::<F>(t18710, t449, t11137, t11247, t14702, t14721, t14723, t14724, t18203, t18208, t18213, t18217, t18219, t18223, t18227, t18229, t18234, t18239, t18243);
        let (t18731, t18742, t18747, t18749, t18752, t18755, t18757) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1595::<F>(t1100, t18730, t1107, t11243, t5992, t1102, t4756, t4764, t3287, t5999, t11265, t4748);
        let (t18759, t18761) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1596::<F>(t11211, t11372, t14702, t14705, t14711, t18742, t18747, t18749, t18752, t18755, t18757, t3270, t5999);
        let (t18762, t18783) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1597::<F>(t1102, t18761, t11137, t14818, t18227, t18239, t18497, t18500, t18503, t18508, t18510, t18515, t18518);
        let t18785 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1598::<F>(t11369, t14722, t14766, t14768, t14782, t18203, t18208, t18213, t18217, t18219, t18223, t18229, t18234, t18243, t18494, t18505, t18512, t18521, t18731, t18759, t18762, t18783);
        let t18789 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1599::<F>(t1156, t18785, t11297, t1148, t18676, t18679, t18682, t18685, t18688, t18690, t18692, t18694, t18696, t18711, t3371, t6069, t6085);
    (t18711, t18731, t18742, t18747, t18749, t18752, t18755, t18757, t18762, t18785, t18789)
}
