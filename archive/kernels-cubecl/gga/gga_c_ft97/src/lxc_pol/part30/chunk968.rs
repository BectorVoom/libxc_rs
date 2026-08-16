//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 968/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk968<F: Float>(t1466: F, t34261: F, t681: F, t33993: F, t870: F, t2842: F, t7662: F, t2399: F, t7617: F, t34333: F, t6210: F, t458: F, t7580: F) -> (F, F, F, F, F, F) {
    let t142995 = t1466 * t681 * t34261;
    let t142999 = t33993 * t870;
    let t143002 = t7662 * t2842;
    let t143007 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1466 * t2399 * t7617;
    let t143008 = t6210 * t34333;
    let t143017 = t7580 * t458;
    (t142995, t142999, t143002, t143007, t143008, t143017)
}
