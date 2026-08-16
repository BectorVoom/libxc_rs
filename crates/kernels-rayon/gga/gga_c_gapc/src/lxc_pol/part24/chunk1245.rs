//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1245/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1245(t11198: f64, t1928: f64, t2903: f64, t11199: f64, t8422: f64, t11223: f64, t11257: f64, t1577: f64, t1006: f64, t1603: f64, t3639: f64, t4893: f64) -> (f64, f64, f64, f64, f64) {
    let t35618 = t2903 * t11198 * t1928;
    let t35620 = t8422 * t11199;
    let t35623 = t11257 * t11223 * t1577;
    let t35628 = t1006 * t11223 * t1603;
    let t35631 = t1006 * t3639 * t4893;
    (t35618, t35620, t35623, t35628, t35631)
}
