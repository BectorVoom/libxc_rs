//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta654 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2419;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2420;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2421;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta654(t10750: f64, t13723: f64, t959: f64, t10757: f64, t1580: f64, t41825: f64, t10853: f64, t4483: f64, t13508: f64, t2940: f64, t10713: f64, t10756: f64, t300: f64, t2924: f64, t950: f64, t14369: f64, t13662: f64, t2925: f64, t13724: f64, t13658: f64, t2907: f64, t13716: f64, t2929: f64, t4497: f64, t48762: f64, t48765: f64, t48768: f64, t48770: f64, t49068: f64, t49071: f64, t49075: f64, t49080: f64, t49496: f64, t49499: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49502, t49506, t49508, t49510, t49512, t49513) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2419(t10750, t13723, t959, t10757, t1580, t41825, t10853, t4483, t13508, t2940, t10713, t10756, t300);
        let (t49514, t49517, t49520, t49522, t49525, t49529) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2420(t2924, t950, t14369, t49513, t13662, t2925, t959, t13724, t2940, t13658, t2907, t13716, t2929, t4497);
        let t49530 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2421(t48762, t48765, t48768, t48770, t49068, t49071, t49075, t49080, t49496, t49499, t49502, t49506, t49508, t49510, t49512, t49517, t49520, t49522, t49525, t49529);
    (t49502, t49506, t49508, t49510, t49512, t49514, t49517, t49520, t49522, t49525, t49529, t49530)
}
