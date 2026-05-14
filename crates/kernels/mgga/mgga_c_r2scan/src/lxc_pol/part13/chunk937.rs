//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 937/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk937<F: Float>(t2850: F, t6897: F, t560: F, t8001: F, t481: F, t2182: F, t775: F, t113: F, t7202: F, t253: F, t5134: F, t2185: F, t2562: F, t1234: F, t921: F, t1553: F, t910: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23987 = t2850 * t6897;
    let t24031 = t8001 * t560;
    let t24035 = t8001 * t481;
    let t24039 = t2182 * t775;
    let t24059 = t7202 * t113;
    let t24063 = t5134 * t253;
    let t24064 = t2562 * t2185;
    let t24070 = t921 * t1234;
    let t24118 = t910 * t1553;
    (t23987, t24031, t24035, t24039, t24059, t24063, t24064, t24070, t24118)
}
