//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1306/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1306<F: Float>(t3513: F, t9367: F, t11283: F, t2492: F, t941: F, t11296: F, t2486: F, t31880: F, t31883: F, t31886: F, t31889: F, t31891: F, t31893: F, t31896: F, t31898: F, t31900: F) -> (F, F, F, F) {
    let t31902 = t3513 * t9367;
    let t31905 = t2492 * t11283 * t941;
    let t31907 = t11296 * t2486;
    let t31909 = -0.3560484375e1 * t31880 + 0.142419375e1 * t31883 + 0.1151859375e0 * t31886 - 0.76790625e-1 * t31889 + 0.142419375e1 * t31891 - 0.1898925e1 * t31893 - 0.1898925e1 * t31896 - 0.9494625e0 * t31898 - 0.76790625e-1 * t31900 + 0.3071625e0 * t31902 + 0.3071625e0 * t31905 + 0.15358125e0 * t31907;
    (t31902, t31905, t31907, t31909)
}
