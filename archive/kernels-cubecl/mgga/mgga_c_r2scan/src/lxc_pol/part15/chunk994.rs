//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 994/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk994<F: Float>(t20: F, t5119: F, t3293: F, t2124: F, t7406: F, t10760: F, t7619: F, t6093: F, t7624: F, t2147: F, t3344: F, t980: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11669 = t5119 * t20;
    let t11670 = t3293 * t11669;
    let t11671 = t2124 * t7406;
    let t11672 = t11670 * t11671;
    let t11675 = t10760 * t7619;
    let t11676 = t6093 * t11675;
    let t11678 = t10760 * t7624;
    let t11679 = t2147 * t11678;
    let t11681 = t980 * t3344;
    (t11669, t11670, t11671, t11672, t11675, t11676, t11678, t11679, t11681)
}
