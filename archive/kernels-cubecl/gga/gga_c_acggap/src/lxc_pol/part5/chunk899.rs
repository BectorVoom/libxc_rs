//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 899/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk899<F: Float>(t1032: F, t3531: F, t3761: F, t1036: F, t12254: F, t175: F, t398: F, t1005: F, t121: F, t126: F, t147: F, t7321: F) -> (F, F, F, F, F) {
    let t13371 = t1032 * t3531;
    let t13373 = t1032 * t3761;
    let t13399 = F::cast_from(0.17149607247227894789e-2_f64) * t1036 * t398 * t175 * t12254;
    let t13400 = t1005 * t3761;
    let t13451 = F::cast_from(455.0_f64) / F::cast_from(243.0_f64) * t121 * t7321 * t126 * t147;
    (t13371, t13373, t13399, t13400, t13451)
}
