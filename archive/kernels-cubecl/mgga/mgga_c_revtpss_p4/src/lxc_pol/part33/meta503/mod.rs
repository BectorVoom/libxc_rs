//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta503 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1822;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta503<F: Float>(t1209: F, t7627: F, t460: F, t2142: F, t3555: F, t1204: F, t26936: F, t3801: F, t7669: F, t12587: F, t2155: F, t116: F, t7583: F) -> (F, F, F, F, F, F, F, F) {
        let (t26999, t27008, t27011, t27020, t27025, t27037, t27041, t27060) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1822::<F>(t1209, t7627, t460, t2142, t3555, t1204, t26936, t3801, t7669, t12587, t2155, t116, t7583);
    (t26999, t27008, t27011, t27020, t27025, t27037, t27041, t27060)
}
