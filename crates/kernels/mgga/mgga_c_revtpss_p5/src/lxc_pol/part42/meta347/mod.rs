//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta347 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1155;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1156;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta347<F: Float>(t16088: F, t16094: F, t3169: F, t4820: F, t3188: F, t4817: F, t1065: F, t4772: F, t247: F, t3109: F, t4583: F, t1063: F, t3172: F, t4868: F, t1041: F, t3168: F, t4878: F, t11150: F, t3181: F, t11144: F, t11852: F, t3124: F, t1655: F, t697: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t16095, t16121, t16134, t16138, t16160) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1155::<F>(t16088, t16094, t3169, t4820, t3188, t4817, t1065, t4772, t247, t3109, t4583, t1063);
        let (t16165, t16190, t16199, t16208, t16218, t16219) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1156::<F>(t3172, t4868, t1041, t3168, t4878, t11150, t3181, t11144, t11852, t3124, t4820, t1655, t697);
    (t16095, t16121, t16134, t16138, t16160, t16165, t16190, t16199, t16208, t16218, t16219)
}
