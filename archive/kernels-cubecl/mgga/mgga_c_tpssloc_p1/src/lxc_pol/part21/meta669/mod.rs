//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta669 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2471;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta669<F: Float>(t3030: F, t3481: F, t3032: F, t3505: F, t3514: F, t11147: F, t3439: F, t11789: F, t820: F, t3577: F, t3579: F, t11737: F, t44857: F) -> (F, F, F, F, F, F, F) {
        let (t44927, t44929, t44932, t44938, t44951, t44953, t44965) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2471::<F>(t3030, t3481, t3032, t3505, t3514, t11147, t3439, t11789, t820, t3577, t3579, t11737, t44857);
    (t44927, t44929, t44932, t44938, t44951, t44953, t44965)
}
