//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 991/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk991<F: Float>(t13945: F, t650: F, t13934: F, t2549: F, t2562: F, t38974: F, t883: F, t943: F, t13765: F, t4349: F, t605: F, t13838: F, t5552: F) -> (F, F, F, F, F) {
    let t47766 = F::cast_from(0.10254034973522965712e-1_f64) * t650 * t13945;
    let t47768 = t2549 * t13934;
    let t47772 = t943 * t2562 * t883 * t38974;
    let t47784 = t4349 * t13765 * t605;
    let t47786 = t5552 * t13838;
    (t47766, t47768, t47772, t47784, t47786)
}
