//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 926/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk926(t1253: f64, t4075: f64, t4083: f64, t1229: f64, t4030: f64, t1254: f64, t13589: f64, t1255: f64, t370: f64, t4125: f64, t13562: f64, t4129: f64) -> (f64, f64, f64, f64, f64) {
    let t13702 = t4075 * t4083 * t1253;
    let t13705 = t1229 * t4030;
    let t13708 = t13589 * t1254;
    let t13711 = t1255 * t4075;
    let t13715 = 1.0_f64 / t4125 / t370;
    let t13717 = t13715 * t13562 * t4129;
    (t13702, t13705, t13708, t13711, t13717)
}
