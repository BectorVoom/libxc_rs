//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta591 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2228;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta591<F: Float>(t23740: F, t23753: F, t954: F, t1621: F, t19275: F, t1634: F, t6205: F, t1633: F, t19303: F, t1610: F, t6141: F, t2874: F) -> (F, F, F, F, F, F, F) {
        let (t23754, t23755, t23758, t23761, t23764, t23767, t23769) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2228::<F>(t23740, t23753, t954, t1621, t19275, t1634, t6205, t1633, t19303, t1610, t6141, t2874);
    (t23754, t23755, t23758, t23761, t23764, t23767, t23769)
}
