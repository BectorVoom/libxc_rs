//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta238 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1041;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1042;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta238<F: Float>(t10988: F, t689: F, t2444: F, t887: F, t252: F, t2769: F, t786: F, t2771: F, t676: F, t123: F, t2435: F, t2448: F, t10495: F, t10498: F, t10501: F, t10503: F, t10507: F, t10511: F, t10513: F, t10978: F, t10984: F, t10987: F, t865: F) -> (F, F, F, F, F) {
        let (t10989, t10991, t10992, t10994, t10995, t10997, t10998, t11000) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1041::<F>(t10988, t689, t2444, t887, t252, t2769, t786, t2771, t676, t123, t2435, t2448);
        let t11002 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1042::<F>(t10495, t10498, t10501, t10503, t10507, t10511, t10513, t10978, t10984, t10987, t10989, t10992, t10998, t11000, t865, t887);
    (t10991, t10994, t10995, t10997, t11002)
}
