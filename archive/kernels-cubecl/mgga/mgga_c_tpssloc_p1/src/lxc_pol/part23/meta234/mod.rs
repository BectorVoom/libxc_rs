//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta234 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk885;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta234<F: Float>(t5593: F, t9638: F, t16673: F, t816: F, t13278: F, t1512: F, t5587: F, t9667: F, t120: F, t5611: F, t2639: F, t5619: F) -> (F, F, F, F, F, F) {
        let (t16848, t16872, t16877, t16879, t16891, t16940) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk885::<F>(t5593, t9638, t16673, t816, t13278, t1512, t5587, t9667, t120, t5611, t2639, t5619);
    (t16848, t16872, t16877, t16879, t16891, t16940)
}
