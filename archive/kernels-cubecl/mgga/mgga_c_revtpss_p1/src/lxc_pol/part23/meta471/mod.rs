//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta471 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1921;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta471<F: Float>(t18937: F, t4919: F, t18913: F, t16012: F, t18904: F, t18926: F, t4915: F, t18930: F, t1062: F, t6317: F) -> (F, F, F, F, F, F) {
        let (t19951, t19954, t19957, t19960, t19963, t19968) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1921::<F>(t18937, t4919, t18913, t16012, t18904, t18926, t4915, t18930, t1062, t6317);
    (t19951, t19954, t19957, t19960, t19963, t19968)
}
