//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1267/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1267<F: Float>(t2267: F, t2858: F, t7569: F, t2266: F, t7088: F, t1234: F, t6955: F, t6880: F, t955: F, t2526: F, t7118: F, t97: F, t424: F, t7028: F, t1527: F, t7741: F) -> (F, F, F, F, F, F, F) {
    let t23813 = 18.0 * t2858 * t2267 * t7569;
    let t23816 = 9.0 * t2266 * t2267 * t7088;
    let t23819 = 9.0 * t2266 * t6955 * t1234;
    let t23820 = t6880 * t955;
    let t23823 = 18.0 * t97 * t7118 * t2526;
    let t23824 = t424 * t7028;
    let t23828 = t7741 * t1527;
    (t23813, t23816, t23819, t23820, t23823, t23824, t23828)
}
