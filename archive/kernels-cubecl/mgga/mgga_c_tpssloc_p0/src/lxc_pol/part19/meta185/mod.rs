//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta185 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk837;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk838;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk839;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta185<F: Float>(t225: F, t9725: F, t9877: F, t9908: F, t9935: F, t1891: F, t68: F, t9458: F, t776: F, t845: F, t2553: F, t824: F, t9516: F, t228: F, t230: F, t2667: F, t2672: F, t2675: F, t4225: F, t822: F, t825: F, t232: F, t819: F, t820: F, t2571: F, t2618: F, t2643: F, t2649: F, t2686: F, t817: F, t9642: F, t9649: F, t9653: F, t9657: F, t9663: F, t9668: F, t9672: F, t9675: F, t9679: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t9938, t9947, t9950, t9951, t9954) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk837::<F>(t225, t9725, t9877, t9908, t9935, t1891, t68, t9458, t776, t845, t2553, t824, t9516);
        let t9957 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk838::<F>(t228, t230, t2667, t2672, t2675, t4225, t822, t825, t9938, t9947, t9951, t9954);
        let (t9958, t9960, t9963) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk839::<F>(t232, t9957, t819, t820, t2571, t2618, t2643, t2649, t2686, t817, t9642, t9649, t9653, t9657, t9663, t9668, t9672, t9675, t9679);
    (t9938, t9947, t9950, t9951, t9954, t9957, t9958, t9960, t9963)
}
