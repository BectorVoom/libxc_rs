//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 862/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk862(t1625: f64, t990: f64, t4343: f64, t977: f64, t2979: f64, t4338: f64, t1539: f64, t248: f64, t3051: f64) -> (f64, f64, f64, f64) {
    let t4559 = t990 * t1625;
    let t4562 = t977 * t4343;
    let t4565 = t2979 * t4338;
    let t4571 = t248 * t3051 * t1539;
    (t4559, t4562, t4565, t4571)
}
