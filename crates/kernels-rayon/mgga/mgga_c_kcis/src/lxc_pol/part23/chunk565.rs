//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 565/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk565(t1897: f64, t544: f64, t1319: f64, t5457: f64, t518: f64, t1419: f64, t3786: f64, t1890: f64, t653: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5458 = t544 * t1897;
    let t5459 = t5458 * t1319;
    let t5460 = t5457 * t5459;
    let t5463 = t518 * t1897;
    let t5464 = t5463 * t1419;
    let t5465 = t3786 * t5464;
    let t5469 = t653 * t1890;
    (t5458, t5459, t5460, t5463, t5464, t5465, t5469)
}
