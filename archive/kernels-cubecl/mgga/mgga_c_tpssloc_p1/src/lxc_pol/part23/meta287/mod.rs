//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta287 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk992;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk993;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk994;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk995;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk996;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta287<F: Float>(t4483: F, t5812: F, t1568: F, t5742: F, t2888: F, t10277: F, t20234: F, t2826: F, t136: F, t4337: F, t5398: F, t2768: F, t123: F, t4342: F, t882: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t21107, t21114, t21115, t21118) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk992::<F>(t4483, t5812, t1568, t5742, t2888, t10277, t20234);
        let (t21119, t21120, t21122) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk993::<F>(t21118, t2826, t136, t4337, t5398);
        let (t21123, t21124) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk994::<F>(t21122, t2768, t123);
        let t21126 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk995::<F>(t4342, t5398);
        let (t21127, t21128) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk996::<F>(t21126, t882, t123);
    (t21107, t21114, t21115, t21118, t21119, t21120, t21122, t21123, t21124, t21126, t21127, t21128)
}
