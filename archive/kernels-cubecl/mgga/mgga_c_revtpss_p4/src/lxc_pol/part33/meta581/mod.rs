//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta581 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1992;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1993;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta581<F: Float>(t159: F, t8779: F, t218: F, t816: F, t10685: F, t1946: F, t10671: F, t7033: F, t25255: F, t2689: F, t10690: F, t1945: F, t9646: F, t7030: F, t9789: F, t2453: F, t2783: F, t64: F, t10761: F, t9784: F, t2482: F, t25260: F, t27: F, t596: F, t7036: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t92993, t92996, t92998, t93000, t93001, t93007) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1992::<F>(t159, t8779, t218, t816, t10685, t1946, t10671, t7033, t25255, t2689, t10690, t1945, t9646);
        let (t93008, t93013, t93015, t93016, t93021, t93025, t93034) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1993::<F>(t93007, t7030, t9789, t2453, t2783, t64, t10761, t9784, t2482, t25260, t27, t596, t7036);
    (t92993, t92996, t92998, t93000, t93001, t93008, t93013, t93015, t93016, t93021, t93025, t93034)
}
