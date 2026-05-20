//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta460 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1898;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta460<F: Float>(t16199: F, t19661: F, t1042: F, t1469: F, t4186: F, t4806: F, t16208: F, t1065: F, t6258: F, t906: F, t5825: F, t606: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t19662, t19663, t19666, t19667, t19668, t19671, t19672, t19675, t19676, t19677, t19680) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1898::<F>(t16199, t19661, t1042, t1469, t4186, t4806, t16208, t1065, t6258, t906, t5825, t606);
    (t19662, t19663, t19666, t19667, t19668, t19671, t19672, t19675, t19676, t19677, t19680)
}
