//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 997/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk997(t1280: f64, t433: f64, t1409: f64, t1471: f64, t1317: f64, t1392: f64, t544: f64, t3751: f64, t456: f64, t3752: f64, t518: f64, t3255: f64, t3763: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11228 = t1280 * t1280;
    let t11229 = 1.0_f64 / t11228;
    let t11230 = t433 * t11229;
    let t11322 = t1471 * t1409;
    let t11332 = t1392 * t1317 * t544;
    let t11369 = t3751 * t456 * t544;
    let t11374 = t3752 * t518;
    let t11379 = t3255 * t3763;
    (t11230, t11322, t11332, t11369, t11374, t11379)
}
