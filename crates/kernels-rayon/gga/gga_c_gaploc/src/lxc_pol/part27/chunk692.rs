//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 692/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk692(t123: f64, t1570: f64, t4183: f64, t883: f64, t882: f64, t2344: f64, t4324: f64, t2343: f64, t2304: f64, t4807: f64, t423: f64, t481: f64, t482: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6485 = t1570 * t123;
    let t6486 = t883 * t4183;
    let t6487 = t6485 * t6486;
    let t6488 = t882 * t6487;
    let t6490 = t2344 * t4324;
    let t6491 = t2343 * t6490;
    let t6494 = t2304 * t4807;
    let t6498 = t481 * t482 * t423;
    (t6486, t6488, t6490, t6491, t6494, t6498)
}
