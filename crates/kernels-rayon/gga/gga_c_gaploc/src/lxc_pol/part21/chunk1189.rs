//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1189/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1189(t1559: f64, t1564: f64, t169: f64, t31548: f64, t32033: f64, t10265: f64, t3833: f64, t1063: f64, t2440: f64, t7974: f64, t3818: f64, t3344: f64, t6338: f64) -> (f64, f64, f64, f64, f64) {
    let t32036 = 0.34146007962811379518e0_f64 * t31548 * t1559 * t169 * t1564 * t32033;
    let t32038 = 0.17073003981405689759e0_f64 * t3833 * t10265;
    let t32041 = 0.56910013271352299198e-1_f64 * t1063 * t2440 * t7974;
    let t32043 = 0.2276400530854091968e0_f64 * t3818 * t10265;
    let t32044 = t6338 * t3344;
    (t32036, t32038, t32041, t32043, t32044)
}
