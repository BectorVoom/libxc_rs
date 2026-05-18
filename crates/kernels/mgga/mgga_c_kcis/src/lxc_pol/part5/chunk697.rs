//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 697/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk697<F: Float>(t1094: F, t1795: F, t1172: F, t1195: F, t1816: F, t382: F, t1813: F, t3477: F, t3338: F, t4984: F, t3337: F, t1196: F, t1809: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5082 = t1795 * t1094;
    let t5083 = t5082 * sigma0;
    let t5084 = t5083 * t1172;
    let t5086 = t1195 * t1816;
    let t5087 = t382 * t5086;
    let t5089 = t3477 * t1813;
    let t5091 = t3338 * t4984;
    let t5092 = t3337 * t5091;
    let t5094 = t1809 * t1196;
    (t5082, t5083, t5084, t5086, t5087, t5089, t5091, t5092, t5094)
}
