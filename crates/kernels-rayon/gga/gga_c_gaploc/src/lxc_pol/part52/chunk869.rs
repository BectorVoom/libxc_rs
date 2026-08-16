//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 869/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk869(t2684: f64, t2685: f64, t45466: f64, t11608: f64, t2464: f64, t2465: f64, t2365: f64, t35550: f64, t7630: f64, t13635: f64, t23157: f64, t11844: f64, t2021: f64, t7372: f64) -> (f64, f64, f64, f64, f64) {
    let t45468 = t2684 * t2685 * t45466;
    let t45469 = 0.19171462976960374838e0_f64 * t45468;
    let t45472 = t2684 * t2464 * t2465 * t11608;
    let t45473 = 0.42603251059911944084e-1_f64 * t45472;
    let t45475 = t7630 * t2365 * t35550;
    let t45476 = 0.29792074959875355558e-1_f64 * t45475;
    let t45513 = t23157 * t13635;
    let t45516 = t2021 * t11844 * t7372;
    (t45469, t45473, t45476, t45513, t45516)
}
