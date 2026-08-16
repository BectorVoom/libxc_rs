//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta511 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1831;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1832;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta511<F: Float>(t218: F, t816: F, t92993: F, t10685: F, t1946: F, t10671: F, t7033: F, t25255: F, t2689: F, t10680: F, t1945: F, t807: F, t10690: F, t9646: F, t10674: F, t7030: F, t9789: F, t2453: F, t2783: F, t64: F, t10761: F, t9784: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t92995, t92997, t92999, t93001, t93004) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1831::<F>(t218, t816, t92993, t10685, t1946, t10671, t7033, t25255, t2689, t10680, t1945, t807);
        let (t93007, t93010, t93012, t93015, t93016, t93020) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1832::<F>(t10690, t1945, t9646, t10674, t807, t7030, t9789, t2453, t2783, t64, t10761, t9784);
    (t92995, t92997, t92999, t93001, t93004, t93007, t93010, t93012, t93015, t93016, t93020)
}
