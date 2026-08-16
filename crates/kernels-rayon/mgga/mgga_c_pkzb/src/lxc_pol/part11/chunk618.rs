//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 618/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk618(t3426: f64, t83: f64, t1501: f64, t1510: f64, t1555: f64, t1627: f64, t3382: f64, t3421: f64, t3422: f64, t3423: f64, t3424: f64, t3425: f64) -> (f64, f64) {
    let t3427 = t83 * t3426;
    let t3428 = -t3421 + t3422 - t3423 - t3424 - t3425 + t3427 + t3382 + t1627 - t1501 - t1510 - t1555;
    (t3427, t3428)
}
