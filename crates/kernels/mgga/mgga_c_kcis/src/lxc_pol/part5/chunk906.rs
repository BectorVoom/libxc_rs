//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 906/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk906<F: Float>(t1465: F, t540: F, t1494: F, t3754: F, t1392: F, t1457: F, t1017: F, t86: F, t9526: F, t1398: F, t1444: F, t2820: F, t4158: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11823 = t1465 * t1465;
    let t11824 = 1.0 / t11823;
    let t11825 = t540 * t11824;
    let t11826 = t11825 * sigma2;
    let t11854 = t1494 * t3754;
    let t11860 = t1392 * t1457;
    let t11862 = t86 * t1017 * t11860;
    let t11881 = t86 * t9526 * t1392;
    let t11882 = t11881 * t1398;
    let t11898 = t1494 * t1444;
    let t11913 = t86 * t2820 * t4158;
    (t11824, t11825, t11826, t11854, t11862, t11881, t11882, t11898, t11913)
}
