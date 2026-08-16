//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 669/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk669(t3650: f64, t769: f64, t1628: f64, t3676: f64, t3680: f64, t3661: f64, t3666: f64, t1589: f64, t3630: f64, t3670: f64, t3641: f64, t11110: f64, t11120: f64, t2087: f64, t2098: f64, t317: f64, t3642: f64, t3646: f64, t784: f64, t797: f64, t813: f64, t833: f64) -> (f64, f64) {
    let t11936 = t769 * t3650;
    let t11939 = t1628 * t3676;
    let t11942 = t1628 * t3680;
    let t11949 = t1628 * t3661;
    let t11952 = t1628 * t3666;
    let t11955 = t1589 * t3630;
    let t11958 = t1628 * t3670;
    let t11961 = t769 * t3641;
    let t11966 = -0.10725146985555128001e1_f64 * t11936 * t2098 - 0.92023022289409799224e1_f64 * t2087 * t11939 + 0.30674340763136599741e1_f64 * t833 * t11942 + 0.23833659967900284446e0_f64 * t3642 * t784 + 0.23833659967900284446e0_f64 * t3646 * t784 - 0.61348681526273199483e1_f64 * t813 * t11949 + 0.15337170381568299871e2_f64 * t833 * t11952 - 0.23833659967900284446e0_f64 * t797 * t11955 - 0.30674340763136599741e1_f64 * t813 * t11958 + 0.35750489951850426669e0_f64 * t11961 * t317 - 0.76685851907841499353e0_f64 * t11110 + 0.76685851907841499353e0_f64 * t11120;
    (t11936, t11966)
}
