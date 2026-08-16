//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta235 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk886;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta235<F: Float>(t2639: F, t5614: F, t2697: F, t5628: F, t16673: F, t842: F, t5624: F, t13360: F, t1516: F, t5568: F, t9573: F, t2563: F, t5572: F) -> (F, F, F, F, F, F, F) {
        let (t16942, t16954, t16976, t16988, t16990, t16993, t16995) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk886::<F>(t2639, t5614, t2697, t5628, t16673, t842, t5624, t13360, t1516, t5568, t9573, t2563, t5572);
    (t16942, t16954, t16976, t16988, t16990, t16993, t16995)
}
