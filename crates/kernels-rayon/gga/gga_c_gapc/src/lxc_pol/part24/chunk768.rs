//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 768/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk768(t1043: f64, t1668: f64, t3017: f64, t5022: f64, t3157: f64, t8948: f64, t1645: f64, t190: f64, t1649: f64, t1643: f64, t3171: f64, t507: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t9158 = t1043 * t1668;
    let t9160 = t3017 * t5022;
    let t9161 = t1043 * t9160;
    let t9163 = t8948 * t3157;
    let t9166 = t190 * t1645 * pi;
    let t9167 = t9166 * t1649;
    let t9168 = t1643 * t9167;
    let t9173 = t3171 * t507;
    (t9158, t9160, t9161, t9163, t9166, t9168, t9173)
}
