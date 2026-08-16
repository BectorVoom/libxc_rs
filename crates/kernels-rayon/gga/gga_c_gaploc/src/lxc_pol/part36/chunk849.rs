//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 849/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk849(t12886: f64, t4614: f64, t574: f64, t12890: f64, t597: f64, t12762: f64, t1572: f64, t4673: f64, t12922: f64, t26935: f64, t10497: f64, t9285: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41930 = 0.12269736305254639897e2_f64 * t574 * t4614 * t12886;
    let t41933 = 0.58281247449959539508e2_f64 * t597 * t4614 * t12890;
    let t41935 = t597 * t4614 * t12762;
    let t41938 = t1572 * t4673 * t12762;
    let t41941 = 0.42900587942220512003e1_f64 * t26935 * t12922;
    let t41942 = t9285 * t10497;
    (t41930, t41933, t41935, t41938, t41941, t41942)
}
