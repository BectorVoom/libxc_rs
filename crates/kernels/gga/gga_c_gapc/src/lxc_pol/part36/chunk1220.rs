//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1220/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1220<F: Float>(t11391: F, t677: F, t11412: F, t169: F, t4043: F, t8960: F, t11587: F, t27940: F, t2993: F, t11604: F, t27868: F, t33748: F, t8843: F) -> (F, F, F, F, F) {
    let t35259 = t11391 * t677;
    let t35263 = t169 * t11412 * t4043 * t8960;
    let t35266 = t2993 * t11587 * t27940;
    let t35269 = t11604 * t27868;
    let t35272 = t2993 * t33748 * t8843;
    (t35259, t35263, t35266, t35269, t35272)
}
