//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3137/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3137<F: Float>(t43855: F, t43859: F, t43861: F, t43863: F, t44466: F, t50968: F, t50970: F, t50972: F, t50978: F, t64003: F, t64006: F, t64045: F) -> F {
    let t64929 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t64003 - F::cast_from(4.0_f64) * t64006 - t44466 + F::cast_from(5.0_f64) / F::cast_from(81.0_f64) * t43855 + F::cast_from(80.0_f64) / F::cast_from(81.0_f64) * t43859 - F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t43861 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t43863 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t50968 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t50970 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t50972 + F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t50978 - t64045 / F::cast_from(6.0_f64);
    t64929
}
