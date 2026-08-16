//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 939/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk939(t42156: f64, t12871: f64, t8155: f64, t8158: f64, t41878: f64, t6717: f64, t6914: f64, t10532: f64, t10533: f64, t40377: f64, t40392: f64, t40395: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42157 = 0.17875244975925213335e0_f64 * t42156;
    let t42159 = 0.10725146985555128001e1_f64 * t8155 * t12871;
    let t42161 = 0.10725146985555128001e1_f64 * t8158 * t12871;
    let t42163 = t6914 * t6717 * t41878;
    let t42166 = t10532 * t10533 * t41878;
    let t42170 = 0.19171462976960374838e0_f64 * t40377;
    let t42172 = 0.15337170381568299871e1_f64 * t40392;
    let t42173 = 0.29792074959875355558e-1_f64 * t40395;
    (t42157, t42159, t42161, t42163, t42166, t42170, t42172, t42173)
}
