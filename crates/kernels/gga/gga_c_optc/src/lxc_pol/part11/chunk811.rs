//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 811/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk811<F: Float>(t1218: F, t5274: F, t1217: F, t5238: F, t5241: F, t8: F, t5236: F, t1113: F, t190: F, t136: F, t3: F, t496: F) -> (F, F, F, F, F, F) {
    let t15015 = t1218 * t5274;
    let t15016 = t1217 * t15015;
    let t15063 = t5238 * t5241 * t8;
    let t15064 = t5236 * t15063;
    let t15065 = t1113 * t190;
    let t15066 = t15065 * t136;
    let t15067 = t3 * t496;
    (t15015, t15016, t15063, t15064, t15066, t15067)
}
