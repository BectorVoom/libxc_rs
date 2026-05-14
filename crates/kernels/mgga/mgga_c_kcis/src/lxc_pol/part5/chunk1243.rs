//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1243/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1243<F: Float>(t23076: F, t4440: F, t21073: F, t6159: F, t21078: F, t6151: F, t531: F, t7429: F, t833: F, t12605: F, t7492: F, t1610: F, t2104: F, t1889: F, t5440: F, t5426: F) -> (F, F, F, F, F, F, F, F) {
    let t23077 = t4440 * t23076;
    let t23080 = t6159 * t21073;
    let t23083 = t6151 * t21078;
    let t23086 = t7429 * t531;
    let t23087 = t23086 * t833;
    let t23088 = t12605 * t23087;
    let t23091 = t7492 * t531;
    let t23092 = t23091 * t833;
    let t23093 = t4440 * t23092;
    let t23096 = t2104 * t1610;
    let t23097 = t1889 * t23096;
    let t23098 = t12605 * t23097;
    let t23101 = t2104 * t833;
    let t23102 = t5440 * t23101;
    let t23103 = t4440 * t23102;
    let t23106 = t5426 * t23101;
    (t23077, t23080, t23083, t23088, t23093, t23098, t23103, t23106)
}
