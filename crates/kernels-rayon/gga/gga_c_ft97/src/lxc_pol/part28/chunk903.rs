//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 903/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk903(t102: f64, t8416: f64, t100: f64, t480: f64, t8417: f64, t1786: f64, t1825: f64, t24: f64, t32075: f64, t1852: f64, t488: f64, t8216: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t38651 = 1.0_f64 / t8416 / t102;
    let t38652 = t100 * t38651;
    let t38659 = t480 * t8417;
    let t38711 = t1786 * t1825;
    let t38921 = t24 * t32075;
    let t39107 = t1786 * t1852;
    let t39120 = t8216 * t488;
    (t38651, t38652, t38659, t38711, t38921, t39107, t39120)
}
