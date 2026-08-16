//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 843/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk843(t1445: f64, t1562: f64, t2854: f64, t9127: f64, t12886: f64, t4614: f64, t574: f64, t12890: f64, t597: f64, t12922: f64, t26935: f64, t2877: f64, t40251: f64) -> (f64, f64, f64, f64, f64) {
    let t41927 = 0.69017266717057349418e1_f64 * t1562 * t1445 * t2854 * t9127;
    let t41930 = 0.12269736305254639897e2_f64 * t574 * t4614 * t12886;
    let t41933 = 0.58281247449959539508e2_f64 * t597 * t4614 * t12890;
    let t41941 = 0.42900587942220512003e1_f64 * t26935 * t12922;
    let t41945 = 0.35750489951850426669e0_f64 * t40251 * t2877;
    (t41927, t41930, t41933, t41941, t41945)
}
