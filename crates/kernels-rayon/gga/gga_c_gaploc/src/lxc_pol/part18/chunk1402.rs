//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1402/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1402(t2464: f64, t2465: f64, t587: f64, t7980: f64, t1352: f64, t191: f64, t2876: f64, t6604: f64, t1411: f64, t3395: f64, t2365: f64, t2366: f64, t4379: f64, t7892: f64) -> (f64, f64, f64, f64) {
    let t34789 = t587 * t2464 * t2465 * t7980;
    let t34790 = 0.85206502119823888168e-1_f64 * t34789;
    let t34794 = 0.71500979903700853338e0_f64 * t6604 * t2876 * t191 * t1352;
    let t34796 = t587 * t1411 * t3395;
    let t34797 = 0.59644551483876721719e0_f64 * t34796;
    let t34800 = t4379 * t2365 * t2366 * t7892;
    (t34790, t34794, t34797, t34800)
}
