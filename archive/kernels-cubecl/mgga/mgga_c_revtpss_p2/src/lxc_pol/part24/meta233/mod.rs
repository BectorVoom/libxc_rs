//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta233 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk991;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk992;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta233<F: Float>(t1514: F, t2289: F, t1857: F, t3857: F, t2516: F, t5571: F, t1320: F, t5569: F, t2626: F, t1856: F, t2608: F, t512: F, t2496: F, t1317: F, t123: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13448, t13584, t13611, t13621, t13630, t13632, t13633) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk991::<F>(t1514, t2289, t1857, t3857, t2516, t5571, t1320, t5569, t2626, t1856, t2608, t512);
        let (t13652, t13654, t13665) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk992::<F>(t2496, t5571, t1317, t5569, t123, t1856);
    (t13448, t13584, t13611, t13621, t13630, t13632, t13633, t13652, t13654, t13665)
}
