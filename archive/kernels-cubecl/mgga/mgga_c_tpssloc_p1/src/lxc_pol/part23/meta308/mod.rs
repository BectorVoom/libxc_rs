//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta308 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1053;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1054;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1055;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1056;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1057;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta308<F: Float>(t1088: F, t21769: F, t123: F, t21749: F, t1089: F, t20217: F, t11247: F, t14702: F, t18203: F, t18219: F, t18229: F, t21760: F, t21764: F, t21767: F, t1107: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t21770, t21771) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1053::<F>(t1088, t21769, t123);
        let (t21773, t21774) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1054::<F>(t1088, t21749, t123);
        let t21776 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1055::<F>(t1089, t20217);
        let (t21777, t21778) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1056::<F>(t1088, t21776, t123);
        let (t21780, t21781) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1057::<F>(t11247, t14702, t18203, t18219, t18229, t21760, t21764, t21767, t21771, t21774, t21778, t1107);
    (t21770, t21771, t21773, t21774, t21776, t21777, t21778, t21780, t21781)
}
