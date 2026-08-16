//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta337 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1178;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1179;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta337<F: Float>(t11465: F, t23451: F, t3014: F, t981: F, t3011: F, t973: F, t1610: F, t19056: F, t4590: F, t6142: F, t15421: F, t6145: F, t1609: F, t6109: F, t2926: F, t11299: F, t11144: F, t22688: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t23453, t23455, t23457, t23459, t23461, t23463, t23465) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1178::<F>(t11465, t23451, t3014, t981, t3011, t973, t1610, t19056, t4590, t6142, t15421, t6145);
        let (t23466, t23467, t23469, t23470) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1179::<F>(t1609, t6109, t2926, t11299, t11144, t22688);
    (t23453, t23455, t23457, t23459, t23461, t23463, t23465, t23466, t23467, t23469, t23470)
}
