//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1336/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1336(t14626: f64, t3447: f64, t833: f64, t2718: f64, t8556: f64, t11039: f64, t2194: f64, t1445: f64, t2530: f64, t813: f64, t8528: f64, t2949: f64, t7112: f64) -> (f64, f64, f64, f64, f64) {
    let t33905 = 0.51123901271894332903e1_f64 * t833 * t14626 * t3447;
    let t33907 = 0.47667319935800568892e0_f64 * t2718 * t8556;
    let t33912 = 0.92023022289409799224e1_f64 * t2194 * t11039;
    let t33916 = 0.92023022289409799224e1_f64 * t813 * t1445 * t8528 * t2530;
    let t33920 = 0.46011511144704899612e1_f64 * t813 * t1445 * t2949 * t7112;
    (t33905, t33907, t33912, t33916, t33920)
}
