//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 397/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk397(t1570: f64, t551: f64, t552: f64, t537: f64, t774: f64, t255: f64, t571: f64) -> (f64, f64, f64, f64) {
    let t1579 = t551 * t552 * t1570;
    let t1582 = t537 * t774;
    let t1583 = t1582 * t255;
    let t1584 = t571 * t1583;
    (t1579, t1582, t1583, t1584)
}
