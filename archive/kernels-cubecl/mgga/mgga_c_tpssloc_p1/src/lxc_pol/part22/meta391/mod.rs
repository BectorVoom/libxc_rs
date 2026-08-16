//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta391 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1668;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1669;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1670;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1671;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1672;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1673;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1674;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1675;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta391<F: Float>(t18211: F, t3240: F, t123: F, t3966: F, t4723: F, t5976: F, t690: F, t5971: F, t607: F, t1088: F, t4728: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t18212, t18213) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1668::<F>(t18211, t3240, t123);
        let t18215 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1669::<F>(t3966, t4723);
        let (t18216, t18217) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1670::<F>(t18215, t3240, t123);
        let t18219 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1671::<F>(t5976, t690);
        let t18221 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1672::<F>(t5971, t607);
        let (t18222, t18223) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1673::<F>(t1088, t18221, t123);
        let t18225 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1674::<F>(t3966, t4728);
        let (t18226, t18227) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1675::<F>(t1088, t18225, t123);
    (t18212, t18213, t18215, t18216, t18217, t18219, t18221, t18222, t18223, t18225, t18226, t18227)
}
