//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1239/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1239<F: Float>(t114: F, t23040: F, t22671: F, t6817: F, t2054: F, t22666: F, t22644: F, t758: F, t2298: F, t358: F, t364: F, t10645: F, t1779: F, t19092: F, t2049: F, t2056: F, t2096: F, t2104: F, t2279: F, t2289: F, t2292: F, t2295: F, t23: F, t2302: F, t255: F, t269: F, t3428: F, t357: F, t366: F, t513: F, t550: F, t6044: F, t6262: F, t6804: F, t6806: F, t6813: F, t6818: F, t6826: F, t6827: F, t6835: F, t6842: F, t6845: F, t6854: F, t784: F, t786: F, t864: F, t868: F, t870: F, t9: F) -> (F, F, F, F, F) {
    let t23041 = t114 * t23040;
    let t23059 = t6817 * t22671;
    let t23063 = t2054 * t22666;
    let t23067 = t758 * t22644;
    let t23099 = t2298 * t2298;
    let t23102 = t358 / t364 / t23099;
    let t23128 = 8.0 * t6806 * t864 * t6044 - 36.0 * t6817 * t2056 * t6813 - 1.0 * t23067 * t864 - 36.0 * t2289 * t6813 - 6.0 * t6835 * t2049 * t2056 * t357 * t366 - 0.17070113683501086666e0 * t6826 * t6827 * t868 * t2295 - 0.34140227367002173332e0 * t6806 * t3428 * t550 * t513 * t2049 + 0.12765850619867563444e-3 * t10645 * t6854 * t784 * t6262 + 6.0 * t23063 * t864 - 0.59018516198344116898e-5 * t23102 * t2096 / t23 / t1779 * t255 * t550 - 0.22011544503327960294e0 * t2302 * t2104 * t269 / t9 / t786 + 0.13793073340557359535e1 * t870 * t550 * t19092 + 0.34140227367002173332e0 * t6804 * t2292 * t2295 + 0.34140227367002173332e0 * t6818 * t2292 * t2295 - 0.24282398430637586434e-1 * t2279 * t6842 * t6845;
    (t23041, t23059, t23063, t23067, t23128)
}
