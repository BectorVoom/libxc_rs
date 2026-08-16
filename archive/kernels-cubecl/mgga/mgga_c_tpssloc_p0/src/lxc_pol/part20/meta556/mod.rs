//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta556 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2112;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta556<F: Float>(t2403: F, t2830: F, t10317: F, t699: F, t909: F, t9709: F, t10310: F, t2833: F, t2827: F, t10322: F, t10306: F, t10213: F, t241: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t41831, t41833, t41863, t41865, t41870, t41872, t41874, t41876, t41880) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2112::<F>(t2403, t2830, t10317, t699, t909, t9709, t10310, t2833, t2827, t10322, t10306, t10213, t241);
    (t41831, t41833, t41863, t41865, t41870, t41872, t41874, t41876, t41880)
}
