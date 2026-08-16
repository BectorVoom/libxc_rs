//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta440 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1671;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1672;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1673;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1674;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta440<F: Float>(t3555: F, t3727: F, t13180: F, t493: F, t225: F, t3738: F, t3790: F, t1209: F, t13107: F, t460: F, t1269: F, t13043: F, t13038: F, t42859: F, t44376: F, t487: F, t13045: F, t43351: F, t1204: F, t1234: F, t1248: F, t12646: F, t12702: F, t12737: F, t12747: F, t12756: F, t1285: F, t1287: F, t12966: F, t13108: F, t13112: F, t13133: F, t13142: F, t13143: F, t3153: F, t3584: F, t3588: F, t3670: F, t3751: F, t3759: F, t44421: F, t5480: F, t44531: F, t44535: F, t473: F, t17879: F, t44332: F, t1214: F, t12690: F, t12706: F, t12751: F, t12757: F, t1291: F, t13118: F, t13134: F, t17880: F, t3666: F, t3746: F, t3755: F, t3769: F, t3782: F, t3783: F, t44431: F, t44759: F, t5458: F, t17845: F, t17852: F, t12627: F, t3754: F, t17847: F, t17854: F, t17887: F, t12717: F, t12723: F, t12727: F, t12753: F, t13149: F, t17846: F, t17853: F, t44321: F, t44585: F, t44599: F, t44610: F, t44618: F, t44753: F, t490: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t45545, t45552, t45553, t45559, t45568, t45575, t45584) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1671::<F>(t3555, t3727, t13180, t493, t225, t3738, t3790, t1209, t13107, t460, t1269, t13043);
        let (t45609, t45617) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1672::<F>(t13038, t42859, t460, t44376, t487, t13045, t43351, t1204, t1234, t1248, t12646, t12702, t12737, t12747, t12756, t1285, t1287, t12966, t13107, t13108, t13112, t13133, t13142, t13143, t3153, t3584, t3588, t3670, t3727, t3751, t3759, t44421, t45584, t5480);
        let (t45648, t45652) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1673::<F>(t42859, t44531, t460, t43351, t44535, t13107, t473, t1209, t17879, t44332, t487, t1214, t1234, t12690, t12706, t12751, t12756, t12757, t1291, t13118, t13134, t17880, t3666, t3746, t3755, t3769, t3782, t3783, t44431, t44759, t45609, t5458);
        let t45691 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1674::<F>(t1209, t17845, t1214, t13043, t17852, t12627, t3754, t17847, t3588, t17854, t17887, t12717, t12723, t12727, t12753, t1287, t13143, t13149, t17846, t17853, t3755, t44321, t44585, t44599, t44610, t44618, t44753, t490);
    (t45545, t45552, t45553, t45559, t45568, t45575, t45584, t45609, t45617, t45648, t45652, t45691)
}
