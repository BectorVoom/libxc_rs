//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1242/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1242(t33047: f64, t10938: f64, t1980: f64, t2028: f64, t32757: f64, t32970: f64, t326: f64, t32948: f64, t825: f64, t11109: f64, t5840: f64, t10856: f64, t2033: f64, t549: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33048 = 0.29792074959875355558e-1_f64 * t33047;
    let t33055 = 0.79445533226334281486e-1_f64 * t1980 * t10938 * t2028;
    let t33060 = 0.50050685932590597338e1_f64 * t32757 * t32970;
    let t33067 = 0.18404604457881959845e2_f64 * t825 * t326 * t32948;
    let t33068 = t5840 * t11109;
    let t33069 = 0.51123901271894332902e0_f64 * t33068;
    let t33071 = t2033 * t549 * t10856;
    (t33048, t33055, t33060, t33067, t33069, t33071)
}
