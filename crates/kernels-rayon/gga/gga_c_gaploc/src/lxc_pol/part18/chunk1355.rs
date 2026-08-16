//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1355/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1355(t14626: f64, t3354: f64, t597: f64, t10370: f64, t4614: f64, t574: f64, t2859: f64, t30949: f64, t10447: f64, t1562: f64, t10324: f64, t1641: f64) -> (f64, f64, f64, f64, f64) {
    let t34178 = 0.51123901271894332903e1_f64 * t597 * t14626 * t3354;
    let t34181 = 0.12269736305254639897e2_f64 * t574 * t4614 * t10370;
    let t34186 = 0.14300195980740170668e1_f64 * t2859 * t30949;
    let t34189 = 0.18404604457881959845e2_f64 * t1562 * t4614 * t10447;
    let t34191 = 0.12269736305254639897e2_f64 * t1641 * t10324;
    (t34178, t34181, t34186, t34189, t34191)
}
