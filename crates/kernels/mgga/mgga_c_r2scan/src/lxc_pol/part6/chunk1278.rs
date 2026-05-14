//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1278/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1278<F: Float>(t23893: F, t41: F, t88: F, t19687: F, t19729: F, t19735: F, t23949: F, t23951: F, t23954: F, t23956: F, t23959: F, t23961: F, t23964: F, t23968: F, t19743: F, t166: F) -> (F, F, F, F) {
    let t23970 = t41 * t23893 * t88;
    let t23971 = -t23949 - t19687 - t23951 - 0.7089e1 * t19729 + t23954 - t23956 + t23959 - 0.14178e2 * t19735 + t23961 - t23964 - t23968 - t23970;
    let t23972 = 0.17544670867903938621e1 * t19743;
    let t23973 = t23893 * t166;
    (t23970, t23971, t23972, t23973)
}
