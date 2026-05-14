//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1401/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1401<F: Float>(t24163: F, t33097: F, t1873: F, t24248: F, t122193: F, t122195: F, t122197: F, t122199: F, t122201: F, t122203: F, t122205: F, t122207: F, t122209: F, t15851: F, t2591: F, t24182: F, t34368: F) -> (F, F, F, F, F) {
    let t122211 = t33097 * t24163;
    let t122213 = t1873 * t24248;
    let t122215 = -t122193 / 24.0 + 11.0 / 18.0 * t122195 - t122197 / 96.0 - 2.0 / 3.0 * t122199 - 19.0 / 54.0 * t122201 - t122203 / 12.0 - t122205 / 12.0 - t122207 / 12.0 + t122209 / 48.0 + t122211 / 12.0 + 19.0 / 72.0 * t122213;
    let t122217 = t15851 * t2591;
    let t122219 = t34368 * t24182;
    (t122211, t122213, t122215, t122217, t122219)
}
