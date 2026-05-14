//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1443/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1443<F: Float>(t1234: F, t2562: F, t2148: F, t6165: F, t20750: F, t22923: F, t22927: F, t22931: F, t22939: F, t22942: F, t22944: F, t24118: F, t24119: F, t2573: F, t27110: F, t27115: F, t27125: F, t27146: F, t27166: F, t5108: F, t5109: F, t549: F, t551: F, t552: F, t6106: F, t944: F) -> (F,) {
    let t27177 = t2562 * t1234;
    let t27179 = t6165 * t2148 * t27177;
    let t27181 = 0.34672886960217074253e0 * t27110 - 0.25426783770825854452e1 * t22923 - 0.12713391885412927226e1 * t22927 - 0.38140175656238781678e1 * t22931 + 0.69345773920434148506e0 * t27115 - 0.39006997830244208535e0 * t5108 * t5109 * t24118 * t2573 - 0.15602799132097683414e1 * t6106 * t5109 * t24119 - 0.16463622957338778996e-1 * t22939 - 0.14636160809074174528e-1 * t27125 - 0.43341108700271342816e-1 * t549 * t551 * t552 * (t27146 / 2.0 + t27166 / 2.0) - 0.13002332610081402845e0 * t20750 * t944 + 0.2037639021386884617e0 * t22942 - 0.48787202696913915093e-2 * t22944 + 0.52396431978519890151e-1 * t27179;
    (t27181,)
}
