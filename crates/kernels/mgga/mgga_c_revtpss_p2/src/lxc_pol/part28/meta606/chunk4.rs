//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2102/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2102<F: Float>(t2470: F, t27872: F, t25895: F, t1892: F, t7063: F, t25877: F, t25881: F, t1955: F, t97960: F, t14066: F, t213: F, t27960: F) -> (F, F, F, F, F, F, F) {
    let t98028 = t27872 * t2470;
    let t98029 = t25895 * t98028;
    let t98040 = t7063 * t1892;
    let t98041 = t98040 * t25877;
    let t98043 = F::cast_from(0.51405703062096148812e-1_f64) * t98041 * t25881;
    let t98050 = t1955 * t97960;
    let t98053 = t1955 * t14066;
    let t98056 = t213 * t27960;
    (t98028, t98029, t98040, t98043, t98050, t98053, t98056)
}
