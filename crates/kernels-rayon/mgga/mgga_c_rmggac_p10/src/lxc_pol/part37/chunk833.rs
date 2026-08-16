//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 833/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk833(t1587: f64, t234: f64, t3157: f64, t69064: f64, t69069: f64, t13949: f64, t6355: f64, t13975: f64, t38530: f64, t27: f64, t9169: f64, t16058: f64, t69609: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t74996 = t234 * t1587;
    let t74997 = t74996 * t3157;
    let t75002 = 0.39726959900411316772e-4_f64 * t69064;
    let t75003 = 0.19863479950205658386e-4_f64 * t69069;
    let t75005 = 0.5987120850931904282e-1_f64 * t6355 * t13949;
    let t75006 = t38530 * t13975;
    let t75008 = t27 * t9169;
    let t75010 = t69609 * t16058 * t75008;
    (t74997, t75002, t75003, t75005, t75006, t75010)
}
