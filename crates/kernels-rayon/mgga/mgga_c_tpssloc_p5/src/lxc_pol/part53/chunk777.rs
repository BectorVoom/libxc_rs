//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 777/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk777(t10143: f64, t2056: f64, t2094: f64, t3701: f64, t112: f64, t7222: f64, t111: f64, t2098: f64, t191: f64, t192: f64, t5118: f64, t1390: f64, t5187: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24344 = t2056 * t10143;
    let t24432 = t2094 * t3701;
    let t24462 = t7222 * t112;
    let t24465 = t2098 * t111;
    let t24987 = t5118 * t191 * t192;
    let t24990 = t1390 * t5187;
    (t24344, t24432, t24462, t24465, t24987, t24990)
}
