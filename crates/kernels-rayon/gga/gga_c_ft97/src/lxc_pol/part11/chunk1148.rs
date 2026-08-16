//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1148/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1148(t44081: f64, t44096: f64, t44113: f64, t44128: f64, t871: f64, t10705: f64, t8392: f64, t2864: f64, t8232: f64, t10685: f64, t1882: f64, t10668: f64) -> (f64, f64, f64, f64, f64) {
    let t44131 = t871 * (t44081 + t44096 + t44113 + t44128);
    let t44135 = t8392 * t10705;
    let t44145 = t8232 * t2864;
    let t44147 = t1882 * t10685;
    let t44149 = t1882 * t10668;
    (t44131, t44135, t44145, t44147, t44149)
}
