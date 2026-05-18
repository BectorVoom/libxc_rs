//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1318/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1318<F: Float>(t14538: F, t3792: F, t51328: F, t56855: F, t56857: F, t56859: F, t56861: F, t56863: F, t56865: F, t56867: F, t56869: F, t56871: F, t56873: F, t56877: F) -> F {
    let t56880 = t14538 * t51328 * t3792;
    let t56882 = F::new(7.0) / F::new(288.0) * t56855 + t56857 / F::new(24.0) - t56859 / F::new(192.0) - t56861 / F::new(192.0) - t56863 / F::new(768.0) - t56865 / F::new(768.0) + t56867 / F::new(192.0) - t56869 / F::new(96.0) + t56871 / F::new(192.0) + t56873 / F::new(192.0) + t56877 / F::new(48.0) + F::new(7.0) / F::new(288.0) * t56880;
    t56882
}
