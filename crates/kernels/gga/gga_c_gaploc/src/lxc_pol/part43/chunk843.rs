//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 843/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk843<F: Float>(t1445: F, t1562: F, t2854: F, t9127: F, t12886: F, t4614: F, t574: F, t12890: F, t597: F, t12922: F, t26935: F, t2877: F, t40251: F) -> (F, F, F, F, F) {
    let t41927 = F::new(0.69017266717057349418e1) * t1562 * t1445 * t2854 * t9127;
    let t41930 = F::new(0.12269736305254639897e2) * t574 * t4614 * t12886;
    let t41933 = F::new(0.58281247449959539508e2) * t597 * t4614 * t12890;
    let t41941 = F::new(0.42900587942220512003e1) * t26935 * t12922;
    let t41945 = F::new(0.35750489951850426669e0) * t40251 * t2877;
    (t41927, t41930, t41933, t41941, t41945)
}
