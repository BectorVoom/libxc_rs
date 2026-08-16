//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1326/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1326(t10930: f64, t10931: f64, t32948: f64, t2615: f64, t326: f64, t33155: f64, t10847: f64, t16036: f64, t7630: f64, t33557: f64, t10851: f64, t7375: f64) -> (f64, f64, f64, f64, f64) {
    let t33702 = 0.55213813373645879534e2_f64 * t10930 * t10931 * t32948;
    let t33705 = 0.46011511144704899612e1_f64 * t2615 * t326 * t33155;
    let t33708 = 0.95334639871601137784e0_f64 * t7630 * t16036 * t10847;
    let t33711 = 0.92023022289409799224e1_f64 * t2615 * t326 * t33557;
    let t33713 = 0.92023022289409799224e1_f64 * t7375 * t10851;
    (t33702, t33705, t33708, t33711, t33713)
}
