//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta440 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1671;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1672;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1673;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1674;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta440(t3555: f64, t3727: f64, t13180: f64, t493: f64, t225: f64, t3738: f64, t3790: f64, t1209: f64, t13107: f64, t460: f64, t1269: f64, t13043: f64, t13038: f64, t42859: f64, t44376: f64, t487: f64, t13045: f64, t43351: f64, t1204: f64, t1234: f64, t1248: f64, t12646: f64, t12702: f64, t12737: f64, t12747: f64, t12756: f64, t1285: f64, t1287: f64, t12966: f64, t13108: f64, t13112: f64, t13133: f64, t13142: f64, t13143: f64, t3153: f64, t3584: f64, t3588: f64, t3670: f64, t3751: f64, t3759: f64, t44421: f64, t5480: f64, t44531: f64, t44535: f64, t473: f64, t17879: f64, t44332: f64, t1214: f64, t12690: f64, t12706: f64, t12751: f64, t12757: f64, t1291: f64, t13118: f64, t13134: f64, t17880: f64, t3666: f64, t3746: f64, t3755: f64, t3769: f64, t3782: f64, t3783: f64, t44431: f64, t44759: f64, t5458: f64, t17845: f64, t17852: f64, t12627: f64, t3754: f64, t17847: f64, t17854: f64, t17887: f64, t12717: f64, t12723: f64, t12727: f64, t12753: f64, t13149: f64, t17846: f64, t17853: f64, t44321: f64, t44585: f64, t44599: f64, t44610: f64, t44618: f64, t44753: f64, t490: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t45545, t45552, t45553, t45559, t45568, t45575, t45584) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1671(t3555, t3727, t13180, t493, t225, t3738, t3790, t1209, t13107, t460, t1269, t13043);
        let (t45609, t45617) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1672(t13038, t42859, t460, t44376, t487, t13045, t43351, t1204, t1234, t1248, t12646, t12702, t12737, t12747, t12756, t1285, t1287, t12966, t13107, t13108, t13112, t13133, t13142, t13143, t3153, t3584, t3588, t3670, t3727, t3751, t3759, t44421, t45584, t5480);
        let (t45648, t45652) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1673(t42859, t44531, t460, t43351, t44535, t13107, t473, t1209, t17879, t44332, t487, t1214, t1234, t12690, t12706, t12751, t12756, t12757, t1291, t13118, t13134, t17880, t3666, t3746, t3755, t3769, t3782, t3783, t44431, t44759, t45609, t5458);
        let t45691 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1674(t1209, t17845, t1214, t13043, t17852, t12627, t3754, t17847, t3588, t17854, t17887, t12717, t12723, t12727, t12753, t1287, t13143, t13149, t17846, t17853, t3755, t44321, t44585, t44599, t44610, t44618, t44753, t490);
    (t45545, t45552, t45553, t45559, t45568, t45575, t45584, t45609, t45617, t45648, t45652, t45691)
}
