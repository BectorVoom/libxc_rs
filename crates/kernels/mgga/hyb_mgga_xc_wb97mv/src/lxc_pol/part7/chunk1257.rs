//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1257/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1257<F: Float>(t2300: F, t4224: F, t11171: F, t819: F, t2322: F, t11035: F, t2194: F, t3352: F, t8966: F, t10995: F, t2217: F, t790: F, t11008: F, t2211: F, t2206: F, t22518: F, t4166: F) -> (F, F, F, F, F, F, F, F) {
    let t30809 = t4224 * t2300;
    let t30816 = t11171 * t819;
    let t30821 = t4224 * t2322;
    let t30831 = 2.0 * t2194 * t11035;
    let t30832 = t3352 * t8966;
    let t30835 = t2217 * t10995 * t790;
    let t30837 = t11008 * t2211;
    let t30840 = t22518 * t4166 * t2206;
    (t30809, t30816, t30821, t30831, t30832, t30835, t30837, t30840)
}
