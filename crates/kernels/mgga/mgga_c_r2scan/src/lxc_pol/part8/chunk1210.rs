//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1210/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1210<F: Float>(t25181: F, t2207: F, t2837: F, t5181: F, t6359: F, t978: F, t1567: F, t2832: F, t255: F, t571: F, t2147: F, t2608: F, t6856: F, t252: F, t550: F, t20303: F, t545: F) -> (F, F, F, F, F, F, F) {
    let t25182 = 0.29634521323209802194e0 * t25181;
    let t25188 = t2207 * t2837 * t5181;
    let t25189 = 0.6112917064160653851e0 * t25188;
    let t25196 = t6359 * t978;
    let t25204 = t1567 * t2832;
    let t25206 = t571 * t25204 * t255;
    let t25210 = t2147 * t6856 * t2608;
    let t25214 = t252 * t255 * t550;
    let t25215 = t545 * t20303 * t25214;
    (t25182, t25189, t25196, t25206, t25210, t25214, t25215)
}
