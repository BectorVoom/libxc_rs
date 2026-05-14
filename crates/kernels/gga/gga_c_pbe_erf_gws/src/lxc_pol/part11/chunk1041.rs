//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1041/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1041<F: Float>(t26308: F, t26314: F, t41334: F, t48046: F, t48049: F, t48050: F, t48052: F, t48056: F, t48059: F, t48060: F, t48062: F, t41339: F, t48067: F, t48069: F, t48071: F, t48076: F, t48078: F, t48080: F, t48082: F, t48084: F, t48086: F, t48088: F) -> (F, F) {
    let t48663 = -t48046 + t48049 - t48050 - t48052 - t48056 + 0.44134814814814814813e-2 * t26308 + 16.0 * t26314 + t48059 + t48060 + 0.43284165449459373508e0 * t41334 - t48062;
    let t48667 = t48067 + t48069 + t48071 + t48076 + t48078 + t48080 + 16.0 / 3.0 * t41339 - t48082 - t48084 + t48086 + t48088;
    (t48663, t48667)
}
