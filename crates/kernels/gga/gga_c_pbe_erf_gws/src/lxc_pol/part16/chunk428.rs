//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 428/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk428<F: Float>(t1597: F, t242: F, t528: F, t700: F, t1354: F, t41: F, t536: F, t1383: F, t148: F, t1472: F, t168: F, t270: F) -> (F, F, F, F, F, F) {
    let t1598 = t1597 * t242;
    let t1601 = F::cast_from(0.16752564107100880375e0_f64) * t528 * t700;
    let t1602 = t41 * t1354;
    let t1605 = t536 * t700;
    let t1608 = F::cast_from(0.83762820535504401876e-1_f64) * t148 * t1383;
    let t1611 = F::cast_from(0.53059442957798955448e-1_f64) * t168 * t1472 * t270;
    (t1598, t1601, t1602, t1605, t1608, t1611)
}
