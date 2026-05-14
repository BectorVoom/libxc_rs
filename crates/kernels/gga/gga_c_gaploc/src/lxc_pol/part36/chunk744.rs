//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 744/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk744<F: Float>(t12890: F, t4614: F, t597: F, t12762: F, t1572: F, t4673: F, t12922: F, t26935: F, t10497: F, t9285: F, t2877: F, t40251: F, t12968: F, t34471: F, t34286: F, t10615: F, t40186: F) -> (F, F, F, F, F, F, F, F, F) {
    let t41933 = 0.58281247449959539508e2 * t597 * t4614 * t12890;
    let t41935 = t597 * t4614 * t12762;
    let t41938 = t1572 * t4673 * t12762;
    let t41941 = 0.42900587942220512003e1 * t26935 * t12922;
    let t41942 = t9285 * t10497;
    let t41945 = 0.35750489951850426669e0 * t40251 * t2877;
    let t41947 = t34471 * t12968;
    let t41948 = 0.89376224879626066675e-1 * t41947;
    let t41949 = t34286 * t12968;
    let t41950 = 0.89376224879626066675e-1 * t41949;
    let t41951 = t10615 * t40186;
    (t41933, t41935, t41938, t41941, t41942, t41945, t41948, t41950, t41951)
}
