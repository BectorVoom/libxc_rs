//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta384 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1812;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1813;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta384<F: Float>(t1179: F, t1188: F, t12547: F, t1196: F, t300: F, t3488: F, t1198: F, t3531: F, t3539: F, t3543: F, t3535: F, t12485: F, t12487: F, t3523: F, t1298: F, t3798: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t12564, t12566, t12571) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1812::<F>(t1179, t1188, t12547, t1196, t300, t3488);
        let (t12573, t12575, t12577, t12579, t12581, t12583, t12584) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1813::<F>(t1198, t12571, t3531, t3539, t3543, t3535, t12485, t12487, t3523, t1196, t1298, t3798);
    (t12564, t12566, t12571, t12573, t12575, t12577, t12579, t12581, t12583, t12584)
}
