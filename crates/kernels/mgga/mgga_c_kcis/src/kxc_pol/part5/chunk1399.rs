//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1399/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1399<F: Float>(t12605: F, t23087: F, t531: F, t7492: F, t833: F, t4440: F, t1610: F, t2104: F, t1889: F, t5440: F, t5426: F, t12617: F) -> (F, F, F, F, F) {
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
    let t23107 = t12617 * t23106;
    (t23088, t23093, t23098, t23103, t23107)
}
