//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta996 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3385;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3386;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta996<F: Float>(t19256: F, t41583: F, t11385: F, t19255: F, t2918: F, t2875: F, t41499: F, t41502: F, t6109: F, t4707: F, t972: F, t4711: F, t52238: F, t5019: F, t11591: F, t6227: F, t6219: F, t19077: F, t914: F, t936: F, t15235: F, t4724: F, t981: F, t41588: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t63589, t63592, t63596, t63597, t63600) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3385::<F>(t19256, t41583, t11385, t19255, t2918, t2875, t41499, t41502, t6109, t4707, t972, t4711, t52238);
        let (t63601, t63607, t63609, t63612, t63615, t63618) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3386::<F>(t5019, t11591, t6227, t6219, t19077, t914, t936, t15235, t4724, t981, t19255, t2875, t41588);
    (t63589, t63592, t63596, t63597, t63600, t63601, t63607, t63609, t63612, t63615, t63618)
}
