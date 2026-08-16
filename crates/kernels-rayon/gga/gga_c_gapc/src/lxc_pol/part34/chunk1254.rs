//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1254/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1254(t11537: f64, t20372: f64, t5059: f64, t1: f64, t1457: f64, t169: f64, t1736: f64, t11344: f64, t11597: f64, t3008: f64, t3060: f64, t1030: f64, t11591: f64, t144: f64, t1461: f64, t8709: f64) -> (f64, f64, f64, f64, f64) {
    let t34921 = t11537 * t20372 * t5059;
    let t34925 = t169 * t1457 * t1736 * t1;
    let t34926 = t34925 * t11344;
    let t34929 = t3060 * t11597 * t3008;
    let t34934 = t1030 * t1461 * t8709 * t144 * t11591;
    (t34921, t34925, t34926, t34929, t34934)
}
