//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta386 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1849;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1850;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1851;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1852;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta386<F: Float>(t3109: F, t4630: F, t4650: F, t884: F, t3071: F, t10436: F, t10441: F, t10449: F, t10455: F, t10460: F, t10490: F, t10496: F, t10504: F, t10511: F, t10517: F, t10863: F, t10866: F, t10871: F, t1618: F, t1622: F, t3048: F, t3070: F, t4636: F, t3108: F, t4640: F, t1611: F, t3047: F, t3103: F, t4641: F, t1040: F, t4616: F) -> (F, F, F, F, F, F, F, F) {
        let (t14059, t14068, t14069, t14074) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1849::<F>(t3109, t4630, t4650, t884, t3071, t10436, t10441, t10449, t10455, t10460, t10490, t10496, t10504, t10511, t10517, t10863, t10866, t10871, t1618, t1622, t3048, t3070, t4636);
        let t14077 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1850::<F>(t3108, t4640);
        let t14080 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1851::<F>(t1611, t3047);
        let (t14084, t14085) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1852::<F>(t3103, t4641, t1040, t4616);
    (t14059, t14068, t14069, t14074, t14077, t14080, t14084, t14085)
}
