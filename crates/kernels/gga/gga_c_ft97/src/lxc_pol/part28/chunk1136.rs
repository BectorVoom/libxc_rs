//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1136/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1136<F: Float>(t23925: F, t28: F, t6615: F, t89: F, t2185: F, t27157: F, t27158: F, t32924: F, t32962: F, t3424: F, t139431: F, t32897: F) -> (F, F, F, F) {
    let t148396 = t89 * t28 * t23925 * t6615;
    let t148401 = t27157 * t2185 * t32924 * t27158;
    let t148403 = t32962 * t3424;
    let t148405 = t32897 * t139431 * t148403;
    (t148396, t148401, t148403, t148405)
}
