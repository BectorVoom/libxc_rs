//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1135/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1135<F: Float>(t1105: F, t13701: F, t18899: F, t18961: F, t18968: F, t18970: F, t18973: F, t18977: F, t48510: F, t48511: F, t48512: F, t48513: F, t48514: F, t48515: F, t48516: F, t49430: F, t49431: F, t49432: F, t804: F) -> (F,) {
    let t50771 = 12.0 * t1105 * t13701 * t804 - t18899 - t18961 - t18968 + t18970 + t18973 - t18977 + t48510 - t48511 + t48512 - t48513 + t48514 - t48515 - t48516 - t49430 + t49431 + t49432;
    (t50771,)
}
