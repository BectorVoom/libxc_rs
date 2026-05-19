//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1182/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1182<F: Float>(t32097: F, t41184: F, t47825: F, t47828: F, t47832: F, t47836: F, t47839: F, t47841: F, t47844: F, t47848: F, t47850: F, t47851: F) -> F {
    let t48651 = -t47825 - t47828 - t47832 - t47836 + t47839 - t47841 - t47844 - t47848 - t47850 - t47851 + F::cast_from(0.43284165449459373508e0_f64) * t32097 + F::new(16.0) / F::new(3.0) * t41184;
    t48651
}
