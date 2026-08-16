//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta165 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk788;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk789;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk790;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk791;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk792;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta165<F: Float>(t2369: F, t745: F, t180: F, t2511: F, t9489: F, t40: F, t52: F, t761: F, t607: F, t75: F, t2250: F, t634: F, t767: F, t9258: F, t9288: F, t78: F, t638: F, t771: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
        let t9490 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk788::<F>(t2369, t745);
        let t9493 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk789::<F>(t180, t2511);
        let t9494 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk790::<F>(t9489, t9490, t9493);
        let (t9496, t9499, t9505, t9508, t9514) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk791::<F>(t40, t52, t761, t9494, t607, t75, t2250, t634, t767, t9258, t9288, t78, t638, t771, zeta_threshold);
        let t9516 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk792::<F>(t9505, t9514);
    (t9490, t9493, t9494, t9496, t9499, t9508, t9516)
}
