//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1137/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1137(t5842: f64, t5916: f64, t920: f64, t139352: f64, t32897: f64, t139248: f64, t3188: f64, t139329: f64, t2258: f64, t7369: f64, t148288: f64, t26950: f64, t5899: f64, t5900: f64, t9432: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
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
