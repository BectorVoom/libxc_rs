//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1004/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1004<F: Float>(t1849: F, t601: F, t6347: F, t6405: F, t2002: F, t518: F, t596: F, t84: F) -> (F, F, F) {
    let t22111 = F::cast_from(0.62336721237753107879e3_f64) * t601 * t6405 * t1849 * t6347;
    let t22115 = F::cast_from(0.18989760778855128827e-2_f64) * t596 * t518 * t2002 * t84;
    let t22120 = t1849 * t1849;
    (t22111, t22115, t22120)
}
