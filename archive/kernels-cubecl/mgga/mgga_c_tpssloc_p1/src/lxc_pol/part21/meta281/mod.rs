//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta281 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1563;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1564;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1565;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1566;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta281<F: Float>(t2553: F, t262: F, t261: F, t2751: F, t225: F, t3023: F, t1053: F, t68: F, t3021: F, t1887: F, t337: F, t615: F, t134: F, t976: F, t984: F, t2990: F, t2986: F, t271: F, t2775: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10130, t10143, t10160, t10163, t10164, t10165, t10170, t10186) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1563::<F>(t2553, t262, t261, t2751, t225, t3023, t1053, t68, t3021, t1887, t337, t615);
        let t10189 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1564::<F>(t134, t976);
        let t10190 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1565::<F>(t10189, t984);
        let (t10192, t10213) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1566::<F>(t10190, t2990, t2986, t271, t2775);
    (t10130, t10143, t10160, t10163, t10164, t10165, t10170, t10186, t10189, t10190, t10192, t10213)
}
