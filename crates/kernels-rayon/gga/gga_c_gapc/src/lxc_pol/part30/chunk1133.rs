//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1133/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1133(t1: f64, t33549: f64, t128: f64, t18639: f64, t941: f64, t2660: f64, t24759: f64, t667: f64, t277: f64, t11980: f64, t11772: f64, t29006: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t33958 = t33549 * t1;
    let t33961 = t18639 * t941 * t128;
    let t33962 = t2660 * t33958 * t33961;
    let t33965 = t667 * t24759 * pi;
    let t33966 = t277 * t33965;
    let t33967 = t33966 * t11980;
    let t33969 = t11772 * t29006;
    (t33958, t33961, t33962, t33965, t33966, t33967, t33969)
}
