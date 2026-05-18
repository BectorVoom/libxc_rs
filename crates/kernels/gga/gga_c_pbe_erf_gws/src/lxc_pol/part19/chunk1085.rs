//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1085/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1085<F: Float>(t11947: F, t11959: F, t11974: F, t11976: F, t11979: F, t11983: F, t11986: F, t11989: F, t12002: F, t12005: F, t6597: F, t9123: F, t9142: F) -> F {
    let t12159 = t9123 + t11947 - t6597 + t11959 + t11974 - t9142 - t11976 - t11979 - t11983 - t11986 + t11989 - t12002 - t12005;
    t12159
}
