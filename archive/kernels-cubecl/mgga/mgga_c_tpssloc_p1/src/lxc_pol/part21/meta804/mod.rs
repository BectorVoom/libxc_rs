//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta804 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2792;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta804<F: Float>(t59022: F, t12924: F, t16693: F, t13127: F, t16616: F, t2528: F, t12908: F, t16620: F, t12932: F, t4205: F, t47180: F, t47185: F) -> (F, F, F, F, F, F, F, F) {
        let (t59023, t59025, t59027, t59029, t59031, t59033, t59034, t59035) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2792::<F>(t59022, t12924, t16693, t13127, t16616, t2528, t12908, t16620, t12932, t4205, t47180, t47185);
    (t59023, t59025, t59027, t59029, t59031, t59033, t59034, t59035)
}
