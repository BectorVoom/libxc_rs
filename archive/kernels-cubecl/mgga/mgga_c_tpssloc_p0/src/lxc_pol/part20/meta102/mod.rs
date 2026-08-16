//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta102 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk686;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk687;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk688;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk689;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk690;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta102<F: Float>(t2531: F, t763: F, t2504: F, t739: F, t746: F, t40: F, t52: F, t761: F, t718: F, t751: F, t2244: F, t2250: F, t75: F, t767: F, t771: F, t78: F, zeta_threshold: F, t15: F, t60: F, t59: F, t207: F, t215: F, t782: F, t786: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t2532, t2533, t2535) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk686::<F>(t2531, t763, t2504, t739, t746);
        let (t2537, t2538, t2539, t2553) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk687::<F>(t40, t52, t2535, t761, t718, t751, t2244, t2250, t75, t767, t771, t78, zeta_threshold);
        let t2558 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk688::<F>(t15, t60);
        let t2559 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk689::<F>(t2558, t59);
        let (t2562, t2563) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk690::<F>(t207, t215, t2559, t782, t786);
    (t2532, t2533, t2535, t2537, t2538, t2539, t2553, t2558, t2559, t2562, t2563)
}
