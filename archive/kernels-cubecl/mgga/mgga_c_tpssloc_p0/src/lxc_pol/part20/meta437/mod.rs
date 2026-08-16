//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta437 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1871;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1872;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1873;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1874;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1875;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1876;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1877;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta437<F: Float>(t14736: F, t3240: F, t123: F, t2250: F, t4723: F, t2244: F, t1088: F, t3247: F, t3966: F, t607: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14737, t14738) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1871::<F>(t14736, t3240, t123);
        let t14740 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1872::<F>(t2250, t4723);
        let (t14741, t14742) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1873::<F>(t14740, t3240, t123);
        let t14744 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1874::<F>(t2244, t4723);
        let (t14745, t14746) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1875::<F>(t1088, t14744, t123);
        let (t14748, t14749) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1876::<F>(t3247, t3966, t607);
        let (t14750, t14751) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1877::<F>(t1088, t14749, t123);
    (t14737, t14738, t14740, t14741, t14742, t14744, t14745, t14746, t14748, t14749, t14750, t14751)
}
