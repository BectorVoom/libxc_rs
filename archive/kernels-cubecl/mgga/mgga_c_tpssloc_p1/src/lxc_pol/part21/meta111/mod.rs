//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta111 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk764;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk765;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk766;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk767;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk768;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk769;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk770;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk771;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk772;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk773;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta111<F: Float>(t868: F, t261: F, t193: F, t202: F, t2486: F, t2522: F, t2523: F, t2530: F, t2533: F, t2537: F, t2539: F, t2553: F, t2654: F, t2657: F, t2661: F, t2665: F, t2745: F, t766: F, t776: F, t870: F, t2521: F, t1878: F, t268: F, t271: F, t690: F, t885: F, t1043: F, t154: F, t632: F, t2244: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t2749 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk764::<F>(t868);
        let (t2751, t2752) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk765::<F>(t261);
        let t2755 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk766::<F>(t193, t202, t2486, t2522, t2523, t2530, t2533, t2537, t2539, t2553, t2654, t2657, t2661, t2665, t2745, t2749, t2752, t766, t776, t870);
        let t2756 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk767::<F>(t2521, t2755);
        let t2764 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk768::<F>(t1878, t268, t271);
        let (t2765, t2766) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk769::<F>(t2764, t690, t885);
        let t2768 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk770::<F>(t1043, t154);
        let t2769 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk771::<F>(t632);
        let t2770 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk772::<F>(t2769);
        let t2771 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk773::<F>(t2244, t2770);
    (t2749, t2751, t2752, t2756, t2764, t2765, t2766, t2768, t2769, t2770, t2771)
}
