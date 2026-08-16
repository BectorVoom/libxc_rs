//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 777/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk777(t32545: f64, t492: f64, t83: f64, t1332: f64, t22940: f64, t452: f64, t5710: f64, t5722: f64, t110: f64, t32077: f64, t8411: f64, t7165: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32546 = t32545 * t492;
    let t32547 = t83 * t32546;
    let t32550 = t22940 * t1332;
    let t32551 = t83 * t32550;
    let t32555 = t452 * t5710 * t5722;
    let t32559 = t8411 * t110 * t32077;
    let t32562 = t7165 * t492;
    (t32546, t32547, t32550, t32551, t32555, t32559, t32562)
}
