//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 750/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk750<F: Float>(t21369: F, t319: F, t835: F, t10758: F, t21351: F, t1255: F, t4973: F, t2857: F, t4965: F, t1212: F, t4917: F, t4265: F, t2874: F, t10503: F, t22186: F, t2881: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22218 = t835 * t319 * t21369;
    let t22222 = t10758 * t319 * t21351;
    let t22226 = t835 * t1255 * t4973;
    let t22230 = t2857 * t1255 * t4965;
    let t22240 = t4917 * t1212;
    let t22241 = t4265 * t22240;
    let t22242 = t2874 * t22241;
    let t22245 = t10503 * t22186;
    let t22246 = t2881 * t22245;
    (t22218, t22222, t22226, t22230, t22240, t22241, t22242, t22245, t22246)
}
