//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1421/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1421(t12161: f64, t835: f64, t12223: f64, t1880: f64, t325: f64, t723: f64, t12225: f64, t12232: f64, t12252: f64, t12256: f64, t12263: f64, t1445: f64, t1998: f64, t2194: f64, t2201: f64, t32944: f64, t32946: f64, t32951: f64, t32953: f64, t32955: f64, t32958: f64, t32960: f64, t32963: f64, t5694: f64, t5703: f64, t6159: f64, t701: f64, t7653: f64, t813: f64) -> (f64, f64, f64, f64, f64) {
    let t38961 = t835 * t12161;
    let t38970 = t12223 * t1880;
    let t38974 = t325 * t12161;
    let t38975 = t38974 * t723;
    let t38983 = t32944 + t32946 - t32951 - 0.46011511144704899612e1_f64 * t6159 * t12232 - 0.46011511144704899612e1_f64 * t1998 * t1445 * t38961 * t701 - 0.14300195980740170668e1_f64 * t12256 * t7653 + 0.71500979903700853338e0_f64 * t5703 * t12263 + t32953 + t32955 - t32958 - t32960 + t32963 - 0.46011511144704899612e1_f64 * t2201 * t1445 * t38970 - 0.92023022289409799224e1_f64 * t813 * t1445 * t38975 - 0.92023022289409799224e1_f64 * t2194 * t12225 + 0.92686455430723328401e-1_f64 * t12252 * t5694;
    (t38961, t38970, t38974, t38975, t38983)
}
