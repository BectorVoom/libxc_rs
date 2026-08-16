//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1343/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1343(t34913: f64, t34407: f64, t6716: f64, t6717: f64, t10552: f64, t6974: f64, t10608: f64, t6907: f64, t9272: f64, t10466: f64, t7014: f64, t20843: f64, t2487: f64, t3395: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34914 = 0.59584149919750711116e-1_f64 * t34913;
    let t34917 = 0.13803453343411469884e2_f64 * t6716 * t6717 * t34407;
    let t34919 = 0.92023022289409799224e1_f64 * t6974 * t10552;
    let t34921 = t9272 * t10608 * t6907;
    let t34922 = 0.51762950037793012063e1_f64 * t34921;
    let t34927 = t7014 * t10466;
    let t34928 = 0.51123901271894332902e0_f64 * t34927;
    let t34930 = t2487 * t20843 * t3395;
    (t34914, t34917, t34919, t34922, t34928, t34930)
}
