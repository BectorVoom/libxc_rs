//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1364/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1364<F: Float>(t20994: F, t2553: F, t277: F, t7290: F, t7494: F, t7524: F, t1567: F, t1570: F, t20950: F, t20952: F, t20955: F, t20963: F, t20973: F, t2122: F, t2124: F, t2133: F, t2526: F, t2573: F, t25804: F, t25805: F, t25811: F, t25815: F, t25827: F, t360: F, t495: F, t5109: F, t5110: F, t7197: F) -> (F,) {
    let t25835 = t20994 * t2553;
    let t25836 = 0.12805040077930161442e1 * t25835;
    let t25837 = t277 * t7290;
    let t25843 = t7494 * t7524;
    let t25845 = 0.31205598264195366828e1 * t25804 * t25805 * t5110 + 0.16463622957338778996e-1 * t25811 - 0.1047928639570397803e0 * t25815 - 0.34930954652346593434e0 * t20950 + 0.16463622957338778997e-1 * t20952 + 0.29451592179239371317e0 * t20955 + 0.26004665220162805689e0 * t2133 * t5109 * t7197 * t2573 + 0.38087975358139160777e-1 * t25827 - 0.32927245914677557992e0 * t2122 * t2124 * t1567 * t2526 * t1570 + 0.34672886960217074253e0 * t20963 + t25836 + 0.13002332610081402845e0 * t2133 * t360 * t25837 * t495 - 0.38415120233790484326e0 * t20973 - 0.11524536070137145298e1 * t25843;
    (t25845,)
}
