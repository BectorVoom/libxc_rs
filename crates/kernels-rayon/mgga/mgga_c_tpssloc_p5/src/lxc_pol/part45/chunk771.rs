//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 771/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk771(t6699: f64, t986: f64, t3206: f64, t6705: f64, t6704: f64, t1922: f64, t3016: f64, t2261: f64, t337: f64, t1887: f64, t221: f64, t2987: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23310 = t986 * t6699;
    let t23313 = t6705 * t3206;
    let t23314 = t6704 * t23313;
    let t23317 = t3016 * t1922;
    let t23322 = t2261 * t337;
    let t23323 = t23322 * t1887;
    let t23326 = t221 * t2987;
    (t23310, t23314, t23317, t23322, t23323, t23326)
}
