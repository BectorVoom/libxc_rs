//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 732/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk732<F: Float>(t12579: F, t446: F, t2075: F, t3342: F, t28: F, t89: F, t1017: F, t1986: F, t7368: F, t2223: F, t2983: F) -> (F, F, F, F) {
    let t12580 = t446 * t12579;
    let t12582 = t3342 * t2075;
    let t12584 = t89 * t28 * t12582;
    let t12587 = t7368 * t1017 * t1986;
    let t12589 = t89 * t28 * t12587;
    let t12590 = t2983 * t2223;
    (t12580, t12584, t12589, t12590)
}
