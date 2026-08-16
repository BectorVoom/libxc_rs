//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta473 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1771;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1772;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta473<F: Float>(t1089: F, t491: F, t7327: F, t24574: F, t7365: F, t1235: F, t477: F, t225: F, t7349: F, t7288: F, t7306: F, t3640: F, t7394: F, t11947: F, t2157: F, t111: F, t7263: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t24850, t24851, t24856, t24858, t24880, t24891, t24893, t24905) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1771::<F>(t1089, t491, t7327, t24574, t7365, t1235, t477, t225, t7349, t7288, t7306, t3640, t7394);
        let (t24909, t24932) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1772::<F>(t11947, t2157, t111, t7263);
    (t24850, t24851, t24856, t24858, t24880, t24891, t24893, t24905, t24909, t24932)
}
