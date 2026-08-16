//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta419 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1600;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1601;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1602;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1603;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1604;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1605;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1606;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta419(t11204: f64, t11211: f64, t14702: f64, t14868: f64, t14870: f64, t18742: f64, t18747: f64, t18749: f64, t18752: f64, t18755: f64, t18757: f64, t11137: f64, t14818: f64, t18227: f64, t18239: f64, t18497: f64, t18500: f64, t18503: f64, t18508: f64, t18510: f64, t18515: f64, t18518: f64, t11195: f64, t14720: f64, t14766: f64, t14886: f64, t14890: f64, t18203: f64, t18208: f64, t18213: f64, t18217: f64, t18219: f64, t18223: f64, t18229: f64, t18234: f64, t18243: f64, t18494: f64, t18505: f64, t18512: f64, t18521: f64, t18731: f64, t18762: f64, t1118: f64, t1099: f64, t11185: f64, t6024: f64, t1128: f64, t6031: f64, t11317: f64, t15072: f64, t15074: f64, t11314: f64, t14722: f64, t15083: f64, t15094: f64, t1137: f64, t1147: f64, t6063: f64, t1129: f64, t11303: f64, t11361: f64, t1138: f64, t11415: f64, t1157: f64, t15121: f64, t15141: f64, t1683: f64, t1695: f64, t3327: f64, t4797: f64, t4820: f64, t4835: f64, t4858: f64, t6037: f64, t6053: f64, t6056: f64, t6088: f64, t18630: f64, t18673: f64, t18789: f64, t300: f64, t3400: f64, t6084: f64, t4883: f64, t1164: f64, t18247: f64, t18249: f64, t18251: f64, t18257: f64, t18261: f64, t18264: f64, t18268: f64, t18270: f64, t18273: f64, t18278: f64, t18282: f64, t18285: f64, t18672: f64, t18676: f64, t18679: f64, t1166: f64, t4874: f64, t3411: f64, t6098: f64, t4869: f64, t4884: f64, t1156: f64, t18785: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18810, t18832) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1600(t11204, t11211, t14702, t14868, t14870, t18742, t18747, t18749, t18752, t18755, t18757, t11137, t14818, t18227, t18239, t18497, t18500, t18503, t18508, t18510, t18515, t18518);
        let t18834 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1601(t11195, t14720, t14766, t14886, t14890, t18203, t18208, t18213, t18217, t18219, t18223, t18229, t18234, t18243, t18494, t18505, t18512, t18521, t18731, t18762, t18810, t18832);
        let (t18837, t18839, t18840, t18869) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1602(t1118, t18834, t1099, t11185, t6024, t1128, t6031, t11211, t11317, t14702, t15072, t15074, t18742, t18747, t18749, t18752, t18755, t18757);
        let t18893 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1603(t11137, t14818, t18227, t18239, t18497, t18500, t18503, t18508, t18510, t18515, t18518, t11314, t14722, t14766, t15083, t15094, t18203, t18208, t18213, t18217, t18219, t18223, t18229, t18234, t18243, t18494, t18505, t18512, t18521, t18731, t18762, t18869);
        let t18906 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1604(t1137, t18893, t1147, t6063, t1129, t11303, t11361, t1138, t11415, t1157, t15121, t15141, t1683, t1695, t18837, t18839, t18840, t3327, t4797, t4820, t4835, t4858, t6037, t6053, t6056, t6088);
        let (t18909, t18913, t18914) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1605(t18630, t18673, t18789, t18906, t300, t3400, t6084, t4883, t1164, t18247, t18249, t18251, t18257, t18261, t18264, t18268, t18270, t18273, t18278, t18282, t18285, t18672, t18676, t18679);
        let (t18917, t18920, t18922, t18924, t18926) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1606(t300, t6063, t1166, t4858, t4874, t1164, t3411, t6098, t4869, t4884, t1147, t1156, t18785);
    (t18837, t18839, t18909, t18913, t18914, t18917, t18920, t18922, t18924, t18926)
}
