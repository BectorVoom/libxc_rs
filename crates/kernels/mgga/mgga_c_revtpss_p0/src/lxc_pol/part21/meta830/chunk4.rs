//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3098/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3098<F: Float>(t3584: F, t5341: F, t1214: F, t17711: F, t12773: F, t17605: F, t1261: F, t17557: F, t3172: F, t17535: F, t3711: F, t17728: F, t3555: F, t489: F) -> (F, F, F, F, F, F) {
    let t56825 = t5341 * t3584;
    let t56830 = t17711 * t1214;
    let t56835 = t17605 * t12773;
    let t56838 = t1261 * t3172 * t17557;
    let t56853 = t3711 * t3172 * t17535;
    let t56861 = t3555 * t489 * t17728;
    (t56825, t56830, t56835, t56838, t56853, t56861)
}
