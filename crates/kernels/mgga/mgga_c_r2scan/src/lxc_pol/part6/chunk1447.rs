//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1447/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1447<F: Float>(t2182: F, t25214: F, t6091: F, t2562: F, t264: F, t2133: F, t2294: F, t8102: F, t6139: F, t7368: F, t1567: F, t2122: F, t2124: F, t23014: F, t23018: F, t23020: F, t23025: F, t24877: F, t2545: F, t2557: F, t2567: F, t2573: F, t2591: F, t360: F, t5074: F, t5110: F, t6149: F, t6171: F, t6370: F, t7194: F, t7987: F, t8003: F, t8103: F) -> (F,) {
    let t27256 = t2182 * t6091 * t25214;
    let t27257 = t264 * t2562;
    let t27273 = t2133 * t2294 * t8102;
    let t27283 = t6139 * t2294 * t7368;
    let t27285 = 0.13002332610081402845e0 * t6149 * t8103 + 0.13002332610081402845e0 * t2133 * t360 * t24877 * t2573 - 0.2037639021386884617e0 * t23014 - t23018 + 0.15602799132097683414e1 * t27256 * t27257 * t5110 + 0.34930954652346593433e-1 * t23020 - 0.7801399566048841707e0 * t6139 * t360 * t2567 * t6370 + 0.54878743191129263322e-1 * t2122 * t2124 * t2545 * t5074 + 0.26004665220162805689e0 * t6149 * t8003 - 0.34672886960217074253e0 * t27273 + 0.39006997830244208535e0 * t7987 * t6171 + 0.16463622957338778996e0 * t2557 * t2124 * t1567 * t7194 * t2591 + 0.20803732176130244552e1 * t27283 + t23025;
    (t27285,)
}
