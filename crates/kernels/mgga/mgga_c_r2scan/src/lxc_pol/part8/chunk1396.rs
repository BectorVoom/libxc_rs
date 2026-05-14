//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1396/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1396<F: Float>(t19839: F, t514: F, t9977: F, t2841: F, t3016: F, t6243: F, t1604: F, t10113: F, t25347: F, t25804: F, t27222: F, t27256: F, t27257: F, t29702: F, t29704: F, t29707: F, t29710: F, t29713: F, t29720: F, t29728: F, t6425: F, t8752: F, t8756: F, t9481: F) -> (F, F) {
    let t33865 = t514 * t19839 * t9977;
    let t33867 = t2841 * t3016;
    let t33868 = t6243 * t33867;
    let t33869 = t1604 * t33868;
    let t33876 = 0.17465477326173296717e-1 * t29702 + 0.19043987679069580389e-1 * t29704 + 0.48787202696913915093e-3 * t29707 - 0.14636160809074174528e-2 * t29710 - 0.87816964854445047168e-1 * t29713 - 0.7801399566048841707e0 * t27222 * t27257 * t8752 + 0.15602799132097683414e1 * t27256 * t27257 * t9481 + 0.31205598264195366828e1 * t25804 * t27257 * t8756 - 0.11708928647259339623e0 * t33865 - 0.49390868872016336991e-1 * t33869 + 0.47709005517312117571e-2 * t25347 + 0.39006997830244208535e0 * t6425 * t10113 - 0.43371823197556470519e-3 * t29720 + 0.69861909304693186866e-1 * t29728;
    (t33868, t33876)
}
