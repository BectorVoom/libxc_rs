//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1777/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1777<F: Float>(t1269: F, t5219: F, t487: F, t5216: F, t1204: F, t1811: F, t1209: F, t5412: F, t17288: F, t116: F, t4292: F, t5883: F, t648: F) -> (F, F, F, F, F, F, F) {
    let t18062 = t5219 * t1269;
    let t18065 = t5216 * t487;
    let t18087 = t1204 * t1811;
    let t18097 = t1209 * t5412;
    let t18114 = t17288 * t487;
    let t18207 = t116 * t4292;
    let t18220 = t648 * t5883;
    (t18062, t18065, t18087, t18097, t18114, t18207, t18220)
}
