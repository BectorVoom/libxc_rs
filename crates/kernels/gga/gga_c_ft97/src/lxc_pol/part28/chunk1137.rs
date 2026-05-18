//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1137/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1137<F: Float>(t5842: F, t5916: F, t920: F, t139352: F, t32897: F, t139248: F, t3188: F, t139329: F, t2258: F, t7369: F, t148288: F, t26950: F, t5899: F, t5900: F, t9432: F) -> (F, F, F, F, F, F, F, F) {
    let t148408 = t5916 * t920 * t5842;
    let t148410 = t32897 * t139352 * t148408;
    let t148412 = t139248 * t3188;
    let t148414 = t32897 * t139352 * t148412;
    let t148417 = t139329 * t3188;
    let t148419 = t32897 * t2258 * t7369 * t148417;
    let t148422 = t32897 * t139352 * t148288;
    let t148426 = t5899 * t9432 * t5900 * t26950;
    (t148408, t148410, t148412, t148414, t148417, t148419, t148422, t148426)
}
