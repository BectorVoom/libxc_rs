//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta117 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk589;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta117<F: Float>(t290: F, t2846: F, t941: F, t945: F, t307: F, t944: F, t302: F, t2904: F, t310: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2925, t2926, t2930, t2938, t2942, t2943, t2950, t2957, t2966, t2967, t2968, t2969) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk589::<F>(t290, t2846, t941, t945, t307, t944, t302, t2904, t310);
    (t2925, t2926, t2930, t2938, t2942, t2943, t2950, t2957, t2966, t2967, t2968, t2969)
}
