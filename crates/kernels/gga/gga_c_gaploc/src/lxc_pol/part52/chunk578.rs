//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 578/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk578<F: Float>(t3650: F, t769: F, t1628: F, t3676: F, t3680: F, t3661: F, t3666: F, t1589: F, t3630: F, t3670: F, t3641: F, t11110: F, t11120: F, t2087: F, t2098: F, t317: F, t3642: F, t3646: F, t784: F, t797: F, t813: F, t833: F) -> (F, F) {
    let t11936 = t769 * t3650;
    let t11939 = t1628 * t3676;
    let t11942 = t1628 * t3680;
    let t11949 = t1628 * t3661;
    let t11952 = t1628 * t3666;
    let t11955 = t1589 * t3630;
    let t11958 = t1628 * t3670;
    let t11961 = t769 * t3641;
    let t11966 = -0.10725146985555128001e1 * t11936 * t2098 - 0.92023022289409799224e1 * t2087 * t11939 + 0.30674340763136599741e1 * t833 * t11942 + 0.23833659967900284446e0 * t3642 * t784 + 0.23833659967900284446e0 * t3646 * t784 - 0.61348681526273199483e1 * t813 * t11949 + 0.15337170381568299871e2 * t833 * t11952 - 0.23833659967900284446e0 * t797 * t11955 - 0.30674340763136599741e1 * t813 * t11958 + 0.35750489951850426669e0 * t11961 * t317 - 0.76685851907841499353e0 * t11110 + 0.76685851907841499353e0 * t11120;
    (t11936, t11966)
}
