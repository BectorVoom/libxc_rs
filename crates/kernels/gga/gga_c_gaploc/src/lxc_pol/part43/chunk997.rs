//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 997/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk997<F: Float>(t11981: F, t2464: F, t2465: F, t2487: F, t13782: F, t7014: F, t13791: F, t1429: F, t549: F, t40116: F, t1445: F, t1450: F, t447: F, t46919: F) -> (F, F, F, F, F) {
    let t47883 = t2487 * t2464 * t2465 * t11981;
    let t47885 = t7014 * t13782;
    let t47892 = t1429 * t549 * t13791;
    let t47895 = F::cast_from(0.85206502119823888171e-1_f64) * t40116;
    let t47900 = F::cast_from(0.23005755572352449806e1_f64) * t1450 * t1445 * t46919 * t447;
    (t47883, t47885, t47892, t47895, t47900)
}
