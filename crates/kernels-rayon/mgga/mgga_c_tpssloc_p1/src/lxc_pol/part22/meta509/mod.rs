//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta509 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1960;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1961;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1962;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta509(t1683: f64, t6052: f64, t1682: f64, t18643: f64, t6036: f64, t3359: f64, t11314: f64, t11317: f64, t14702: f64, t14766: f64, t18203: f64, t18219: f64, t18229: f64, t18494: f64, t18505: f64, t18512: f64, t21739: f64, t21741: f64, t21747: f64, t21751: f64, t21760: f64, t21764: f64, t21767: f64, t21771: f64, t21774: f64, t21778: f64, t21781: f64, t21783: f64, t21786: f64, t21789: f64, t21792: f64, t21795: f64, t21802: f64, t21804: f64, t1137: f64, t11352: f64, t1671: f64, t6020: f64, t3264: f64, t1129: f64, t11350: f64, t11420: f64, t15146: f64, t1695: f64, t18840: f64, t18899: f64, t21726: f64, t21728: f64, t21812: f64, t21815: f64, t21836: f64, t21839: f64, t3332: f64, t3357: f64, t3376: f64, t3401: f64, t4797: f64, t6053: f64, t6056: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21842, t21845, t21854, t21855, t21870) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1960(t1683, t6052, t1682, t18643, t6036, t3359, t11314, t11317, t14702, t14766, t18203, t18219, t18229, t18494, t18505, t18512, t21739, t21741, t21747, t21751);
        let t21885 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1961(t21760, t21764, t21767, t21771, t21774, t21778, t21781, t21783, t21786, t21789, t21792, t21795, t21802, t21804);
        let (t21886, t21887, t21890, t21895, t21897, t21898) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1962(t21870, t21885, t1137, t11352, t21854, t1671, t6020, t3264, t1129, t11350, t11420, t15146, t1683, t1695, t18840, t18899, t21726, t21728, t21812, t21815, t21836, t21839, t21842, t21845, t21855, t3332, t3357, t3376, t3401, t4797, t6053, t6056);
    (t21842, t21845, t21854, t21855, t21886, t21887, t21890, t21895, t21897, t21898)
}
