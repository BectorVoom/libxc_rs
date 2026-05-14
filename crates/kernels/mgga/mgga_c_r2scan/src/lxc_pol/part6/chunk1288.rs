//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1288/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1288<F: Float>(t1569: F, t24165: F, t20298: F, t6086: F, t2086: F, t2834: F, t113: F, t7204: F, t2147: F, t2698: F, t625: F, t6069: F, t19827: F, t20609: F, t20787: F, t2223: F, t24136: F, t24141: F, t24145: F, t24150: F, t24156: F, t24163: F, t2614: F, t2636: F, t495: F, t506: F, t5109: F, t529: F, t551: F, t552: F, t6139: F, t6425: F, t6528: F, t7544: F, t7576: F) -> (F, F, F) {
    let t24166 = t24165 * t1569;
    let t24168 = t20298 * t6086 * t24166;
    let t24170 = t2834 * t2086;
    let t24171 = 0.12713391885412927226e1 * t24170;
    let t24172 = t7204 * t113;
    let t24174 = t2147 * t6086 * t24172;
    let t24176 = t2698 * t625;
    let t24177 = t6069 * t24176;
    let t24178 = 0.48787202696913915093e-3 * t24177;
    let t24179 = 0.15602799132097683414e1 * t19827 * t5109 * t7576 * t495 - 0.7801399566048841707e0 * t6139 * t5109 * t24136 - 0.69345773920434148506e0 * t24141 + 0.7801399566048841707e0 * t6425 * t7544 - 0.7801399566048841707e1 * t6528 * t551 * t552 * t24145 + 0.34672886960217074253e0 * t24150 + 0.26004665220162805689e0 * t20787 * t2636 + 0.39006997830244208535e0 * t20609 * t2614 + 0.49390868872016336991e0 * t2223 * t529 * t506 * t24156 - 0.58544643236296698111e-1 * t24163 - 0.34930954652346593433e-1 * t24168 + t24171 - 0.17465477326173296717e-1 * t24174 + t24178;
    (t24172, t24176, t24179)
}
