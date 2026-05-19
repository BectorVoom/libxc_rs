//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 849/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk849<F: Float>(t12886: F, t4614: F, t574: F, t12890: F, t597: F, t12762: F, t1572: F, t4673: F, t12922: F, t26935: F, t10497: F, t9285: F) -> (F, F, F, F, F, F) {
    let t41930 = F::cast_from(0.12269736305254639897e2_f64) * t574 * t4614 * t12886;
    let t41933 = F::cast_from(0.58281247449959539508e2_f64) * t597 * t4614 * t12890;
    let t41935 = t597 * t4614 * t12762;
    let t41938 = t1572 * t4673 * t12762;
    let t41941 = F::cast_from(0.42900587942220512003e1_f64) * t26935 * t12922;
    let t41942 = t9285 * t10497;
    (t41930, t41933, t41935, t41938, t41941, t41942)
}
