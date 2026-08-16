//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta626 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2541;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta626<F: Float>(t18937: F, t4919: F, t18913: F, t16012: F, t18904: F, t18926: F, t4915: F, t18930: F, t1062: F, t6317: F, t3154: F, t4866: F) -> (F, F, F, F, F, F, F) {
        let (t19951, t19954, t19957, t19960, t19963, t19968, t19971) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2541::<F>(t18937, t4919, t18913, t16012, t18904, t18926, t4915, t18930, t1062, t6317, t3154, t4866);
    (t19951, t19954, t19957, t19960, t19963, t19968, t19971)
}
