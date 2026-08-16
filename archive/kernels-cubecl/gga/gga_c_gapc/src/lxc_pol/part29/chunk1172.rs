//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1172/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1172<F: Float>(t20774: F, t26312: F, t2993: F, t26597: F, t5395: F, t21072: F, t27408: F, t33539: F, t11308: F, t11329: F, t1036: F, t11488: F, t21111: F) -> (F, F, F, F, F) {
    let t34477 = t2993 * t26312 * t20774;
    let t34480 = t5395 * t26597 * t20774;
    let t34484 = t21072 * t33539 * t27408;
    let t34486 = t11329 * t11308;
    let t34489 = t11488 * t1036 * t21111;
    (t34477, t34480, t34484, t34486, t34489)
}
