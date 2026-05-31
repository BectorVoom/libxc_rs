//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1377/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1377<F: Float>(t15485: F, t840: F, t56855: F, t56857: F, t56859: F, t56861: F, t56863: F, t56865: F, t56867: F, t56869: F, t56871: F, t56873: F, t56877: F, t56880: F) -> (F, F) {
    let t58581 = t840 * t15485;
    let t58596 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t56855 + t56857 / F::cast_from(12.0_f64) - t56859 / F::cast_from(96.0_f64) - t56861 / F::cast_from(96.0_f64) - t56863 / F::cast_from(384.0_f64) - t56865 / F::cast_from(384.0_f64) + t56867 / F::cast_from(96.0_f64) - t56869 / F::cast_from(48.0_f64) + t56871 / F::cast_from(96.0_f64) + t56873 / F::cast_from(96.0_f64) + t56877 / F::cast_from(24.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t56880;
    (t58581, t58596)
}
