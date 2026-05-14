//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1200/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1200<F: Float>(t24074: F, t7625: F, t2195: F, t7244: F, t108: F, t5119: F, t5146: F, t625: F, t7406: F, t1591: F, t2666: F, t6217: F, t2086: F, t2834: F, t2698: F, t6069: F) -> (F, F, F, F, F, F, F, F) {
    let t24075 = t24074 * t7625;
    let t24076 = 0.4939086887201633699e-1 * t24075;
    let t24090 = t2195 * t7244;
    let t24096 = t5146 * t5119 * t108 * t7406 * t625;
    let t24097 = 0.14636160809074174528e-2 * t24096;
    let t24100 = t1591 * t7244;
    let t24107 = t6217 * t2666;
    let t24170 = t2834 * t2086;
    let t24171 = 0.12713391885412927226e1 * t24170;
    let t24176 = t2698 * t625;
    let t24177 = t6069 * t24176;
    (t24076, t24090, t24097, t24100, t24107, t24171, t24176, t24177)
}
