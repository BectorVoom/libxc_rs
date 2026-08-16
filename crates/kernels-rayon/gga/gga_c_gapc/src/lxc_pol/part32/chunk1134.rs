//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1134/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1134(t11971: f64, t277: f64, t34021: f64, t33521: f64, t4052: f64, t1084: f64, t29868: f64, t10079: f64, t33620: f64, t11849: f64, t1952: f64, t919: f64) -> (f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t34023 = t277 * t34021 * t11971;
    let t34026 = t4052 * t33521 * pi;
    let t34028 = t1084 * t34026 * t29868;
    let t34030 = t33620 * t10079;
    let t34033 = t11849 * t1952 * t919;
    (t34023, t34026, t34028, t34030, t34033)
}
