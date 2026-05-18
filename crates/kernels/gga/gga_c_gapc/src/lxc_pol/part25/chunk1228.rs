//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1228/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1228<F: Float>(t20774: F, t26597: F, t5395: F, t21072: F, t27408: F, t33539: F, t11308: F, t11329: F, t1036: F, t11488: F, t21111: F, t1688: F, t21115: F) -> (F, F, F, F, F) {
    let t34480 = t5395 * t26597 * t20774;
    let t34484 = t21072 * t33539 * t27408;
    let t34486 = t11329 * t11308;
    let t34489 = t11488 * t1036 * t21111;
    let t34492 = t11488 * t1688 * t21115;
    (t34480, t34484, t34486, t34489, t34492)
}
