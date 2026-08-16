//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1114/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1114(t35828: f64, t44280: f64, t446: f64, t824: f64, t10683: f64, t6260: f64, t7036: f64, t1882: f64, t35978: f64, t18: f64, t2665: f64, t3281: f64, t33978: f64) -> (f64, f64, f64, f64) {
    let t152948 = t446 * t44280 * t35828 * t824;
    let t152952 = t446 * t10683 * t7036 * t6260;
    let t152954 = t1882 * t35978;
    let t152958 = t3281 * t2665 * t33978 * t18;
    (t152948, t152952, t152954, t152958)
}
