//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta366 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1399;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta366<F: Float>(t14685: F, t220: F, t14671: F, t837: F, t10777: F, t10779: F, t1548: F, t10811: F, t4447: F, t10815: F, t1561: F, t2741: F, t4426: F) -> (F, F, F, F, F, F, F, F) {
        let (t14686, t14688, t14690, t14701, t14703, t14705, t14712, t14715) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1399::<F>(t14685, t220, t14671, t837, t10777, t10779, t1548, t10811, t4447, t10815, t1561, t2741, t4426);
    (t14686, t14688, t14690, t14701, t14703, t14705, t14712, t14715)
}
