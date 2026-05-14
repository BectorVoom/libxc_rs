//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 837/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk837<F: Float>(t1032: F, t3531: F, t3761: F, t1036: F, t12254: F, t175: F, t398: F, t1005: F, t121: F, t126: F, t147: F, t7321: F, t1029: F, t3228: F, t166: F, t1: F) -> (F, F, F, F, F, F, F) {
    let t13371 = t1032 * t3531;
    let t13373 = t1032 * t3761;
    let t13399 = 0.17149607247227894789e-2 * t1036 * t398 * t175 * t12254;
    let t13400 = t1005 * t3761;
    let t13451 = 455.0 / 243.0 * t121 * t7321 * t126 * t147;
    let t13459 = t3228 * t1029;
    let t13461 = t166 * t166;
    let t13462 = 1.0 / t13461;
    let t13463 = t13462 * t1;
    (t13371, t13373, t13399, t13400, t13451, t13459, t13463)
}
