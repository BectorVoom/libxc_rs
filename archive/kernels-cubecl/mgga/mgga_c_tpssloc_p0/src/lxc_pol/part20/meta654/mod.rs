//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta654 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2419;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2420;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2421;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta654<F: Float>(t10750: F, t13723: F, t959: F, t10757: F, t1580: F, t41825: F, t10853: F, t4483: F, t13508: F, t2940: F, t10713: F, t10756: F, t300: F, t2924: F, t950: F, t14369: F, t13662: F, t2925: F, t13724: F, t13658: F, t2907: F, t13716: F, t2929: F, t4497: F, t48762: F, t48765: F, t48768: F, t48770: F, t49068: F, t49071: F, t49075: F, t49080: F, t49496: F, t49499: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t49502, t49506, t49508, t49510, t49512, t49513) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2419::<F>(t10750, t13723, t959, t10757, t1580, t41825, t10853, t4483, t13508, t2940, t10713, t10756, t300);
        let (t49514, t49517, t49520, t49522, t49525, t49529) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2420::<F>(t2924, t950, t14369, t49513, t13662, t2925, t959, t13724, t2940, t13658, t2907, t13716, t2929, t4497);
        let t49530 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2421::<F>(t48762, t48765, t48768, t48770, t49068, t49071, t49075, t49080, t49496, t49499, t49502, t49506, t49508, t49510, t49512, t49517, t49520, t49522, t49525, t49529);
    (t49502, t49506, t49508, t49510, t49512, t49514, t49517, t49520, t49522, t49525, t49529, t49530)
}
