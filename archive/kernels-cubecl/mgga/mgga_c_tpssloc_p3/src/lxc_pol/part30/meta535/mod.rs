//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta535 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1881;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta535<F: Float>(t225: F, t7723: F, t2015: F, t5353: F, t3887: F, t22897: F, t5336: F, t1992: F, t22751: F, t7733: F, t1799: F, t22881: F) -> (F, F, F, F, F, F) {
        let (t26366, t26371, t26378, t26379, t26381, t26384) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1881::<F>(t225, t7723, t2015, t5353, t3887, t22897, t5336, t1992, t22751, t7733, t1799, t22881);
    (t26366, t26371, t26378, t26379, t26381, t26384)
}
