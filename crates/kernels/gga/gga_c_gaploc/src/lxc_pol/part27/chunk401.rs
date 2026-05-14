//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 401/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk401<F: Float>(t165: F, t723: F, t486: F, t1967: F, t1392: F, t325: F, t1391: F, t1402: F, t791: F, t121: F, t769: F) -> (F, F, F, F, F, F) {
    let t1968 = t165 * t723;
    let t1969 = t486 * t1968;
    let t1970 = t1967 * t1969;
    let t1973 = t1392 * t325;
    let t1974 = t1391 * t1973;
    let t1977 = t1402 * t791;
    let t1980 = t769 * t121;
    (t1968, t1969, t1970, t1974, t1977, t1980)
}
