//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta697 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2519;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta697<F: Float>(t12690: F, t1284: F, t1209: F, t17948: F, t3552: F, t3781: F, t1204: F, t13147: F, t13141: F, t3596: F, t42859: F, t460: F) -> (F, F, F, F, F, F, F) {
        let (t45726, t45738, t45764, t45769, t45779, t45785, t45786) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2519::<F>(t12690, t1284, t1209, t17948, t3552, t3781, t1204, t13147, t13141, t3596, t42859, t460);
    (t45726, t45738, t45764, t45769, t45779, t45785, t45786)
}
