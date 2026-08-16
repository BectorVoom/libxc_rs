//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1348/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1348(t10375: f64, t1641: f64, t1445: f64, t25556: f64, t574: f64, t874: f64, t2293: f64, t7980: f64, t2859: f64, t31153: f64, t2877: f64, t6777: f64) -> (f64, f64, f64, f64, f64) {
    let t34070 = 0.92023022289409799224e1_f64 * t1641 * t10375;
    let t34074 = 0.46011511144704899612e1_f64 * t574 * t1445 * t25556 * t874;
    let t34078 = 0.92023022289409799224e1_f64 * t574 * t1445 * t7980 * t2293;
    let t34087 = 0.10725146985555128001e1_f64 * t2859 * t31153;
    let t34092 = 0.35750489951850426669e0_f64 * t6777 * t2877;
    (t34070, t34074, t34078, t34087, t34092)
}
