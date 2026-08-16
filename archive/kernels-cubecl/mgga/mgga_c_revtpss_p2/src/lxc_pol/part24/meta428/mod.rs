//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta428 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1378;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta428<F: Float>(t1209: F, t17852: F, t12627: F, t3754: F, t17948: F, t3596: F, t42859: F, t460: F, t3603: F, t43351: F, t1243: F, t471: F) -> (F, F, F, F, F, F, F) {
        let (t45659, t45666, t45738, t45786, t45787, t45833, t45834) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1378::<F>(t1209, t17852, t12627, t3754, t17948, t3596, t42859, t460, t3603, t43351, t1243, t471);
    (t45659, t45666, t45738, t45786, t45787, t45833, t45834)
}
