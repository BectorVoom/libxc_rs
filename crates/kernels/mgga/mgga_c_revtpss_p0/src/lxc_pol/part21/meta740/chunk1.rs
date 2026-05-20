//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2605/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2605<F: Float>(t47952: F, t10119: F, t14114: F, t10115: F, t1900: F, t14189: F, t2435: F, t22: F, t46389: F, t543: F, t5735: F, t1432: F, t5763: F, t9288: F) -> (F, F, F, F, F, F) {
    let t47953 = F::cast_from(0.21951497276451705329e-1_f64) * t47952;
    let t47957 = t14114 * t10119;
    let t47961 = t10115 * t1900;
    let t47963 = t2435 * t14189;
    let t47964 = F::cast_from(0.21951497276451705329e-1_f64) * t47963;
    let t47967 = t46389 * t5735 * t543 * t22;
    let t47971 = t1432 * t5763 * t9288;
    (t47953, t47957, t47961, t47964, t47967, t47971)
}
