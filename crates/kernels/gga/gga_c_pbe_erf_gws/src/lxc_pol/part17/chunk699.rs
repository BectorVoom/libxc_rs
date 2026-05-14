//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 699/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk699<F: Float>(t1383: F, t528: F, t35: F, t413: F, t1602: F, t700: F, t536: F, t1477: F, t6: F, t153: F, t2704: F, t2718: F, t39: F, t161: F, t1: F, t1368: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4557 = 0.25128846160651320563e0 * t528 * t1383;
    let t4560 = t35 * t413;
    let t4561 = 24.0 * t4560;
    let t4566 = t1602 * t700;
    let t4568 = t536 * t1383;
    let t4573 = t6 * t1477;
    let t4576 = -0.53666666666666666667e-2 * t2704 - 0.60688888888888888888e-1 * t2718 + 0.1829167760955153094e-1 * t39 - 0.36147222222222222223e-2 * t153 * t4573;
    let t4577 = t4576 * t161;
    let t4579 = t1368 * t1;
    (t4557, t4560, t4561, t4566, t4568, t4573, t4576, t4577, t4579)
}
