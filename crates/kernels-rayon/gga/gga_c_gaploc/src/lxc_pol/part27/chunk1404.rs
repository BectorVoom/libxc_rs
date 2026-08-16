//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1404/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1404(t12065: f64, t12093: f64, t1445: f64, t1549: f64, t1555: f64, t1646: f64, t31178: f64, t34928: f64, t34931: f64, t34935: f64, t34937: f64, t34939: f64, t34941: f64, t34944: f64, t34947: f64, t34950: f64, t34953: f64, t3701: f64, t38314: f64, t38436: f64, t4418: f64, t531: f64, t557: f64, t574: f64) -> f64 {
    let t38801 = t31178 - 0.35750489951850426669e0_f64 * t557 * t531 * t38314 - t34928 + t34931 + t34935 - t34937 + 0.51123901271894332905e0_f64 * t4418 * t12093 + t34939 - t34941 + t34944 - t34947 + t34950 + t34953 - 0.92023022289409799224e1_f64 * t574 * t1445 * t38436 - 0.71500979903700853338e0_f64 * t1555 * t3701 * t1646 + 0.71500979903700853338e0_f64 * t1549 * t12065;
    t38801
}
