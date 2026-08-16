//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta671 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2472;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta671<F: Float>(t3057: F, t4995: F, t3143: F, t42859: F, t342: F, t12032: F, t359: F, t3043: F, t3298: F, t16551: F, t994: F, t16558: F) -> (F, F, F, F, F, F, F) {
        let (t43456, t43471, t43472, t43504, t43512, t43520, t43524) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2472::<F>(t3057, t4995, t3143, t42859, t342, t12032, t359, t3043, t3298, t16551, t994, t16558);
    (t43456, t43471, t43472, t43504, t43512, t43520, t43524)
}
