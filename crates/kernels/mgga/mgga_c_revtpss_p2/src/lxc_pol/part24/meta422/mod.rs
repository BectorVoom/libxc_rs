//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta422 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1370;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta422<F: Float>(t13039: F, t44372: F, t44373: F, t13045: F, t42871: F, t3597: F, t3603: F, t3367: F, t1209: F, t13147: F, t17708: F, t12854: F, t17350: F) -> (F, F, F, F, F, F, F) {
        let (t44441, t44442, t44448, t44449, t44458, t44500, t44510) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1370::<F>(t13039, t44372, t44373, t13045, t42871, t3597, t3603, t3367, t1209, t13147, t17708, t12854, t17350);
    (t44441, t44442, t44448, t44449, t44458, t44500, t44510)
}
