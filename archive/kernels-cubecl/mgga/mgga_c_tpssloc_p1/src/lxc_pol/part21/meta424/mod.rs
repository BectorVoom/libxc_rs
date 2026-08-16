//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta424 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1946;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1947;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1948;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta424<F: Float>(t15067: F, t3265: F, t11275: F, t14704: F, t14710: F, t14720: F, t11215: F, t11217: F, t14722: F, t14733: F, t14738: F, t14742: F, t14746: F, t14751: F, t14755: F, t14766: F, t14781: F, t11137: F, t11139: F, t11141: F, t11143: F, t14728: F, t14809: F, t14811: F, t14814: F, t14816: F, t14818: F, t14824: F, t11211: F, t11213: F, t11314: F, t11317: F, t14702: F, t14708: F, t14713: F, t14759: F, t14779: F, t14784: F, t14787: F, t14790: F, t14793: F, t14796: F, t14799: F, t14802: F, t14805: F) -> (F, F, F, F, F, F, F) {
        let (t15068, t15070, t15072, t15074, t15083, t15091) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1946::<F>(t15067, t3265, t11275, t14704, t14710, t14720, t11215, t11217, t14722, t14733, t14738, t14742, t14746, t14751, t14755, t14766);
        let (t15094, t15115) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1947::<F>(t14781, t11137, t11139, t11141, t11143, t14728, t14809, t14811, t14814, t14816, t14818, t14824);
        let t15117 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1948::<F>(t11211, t11213, t11314, t11317, t14702, t14708, t14713, t14759, t14779, t14784, t14787, t14790, t14793, t14796, t14799, t14802, t14805, t15072, t15074, t15091, t15094, t15115);
    (t15068, t15070, t15072, t15074, t15083, t15094, t15117)
}
