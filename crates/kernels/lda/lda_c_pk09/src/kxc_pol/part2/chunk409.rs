//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 409/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk409<F: Float>(t1971: F, t451: F, t1754: F, t1765: F, t1684: F, t1735: F, t1732: F, t1738: F, t1762: F, t1769: F, t447: F, t452: F) -> (F, F, F, F, F, F, F, F) {
    let t2091 = t451 * t1971;
    let t2094 = F::cast_from(1.4770435158815312_f64) * t1754;
    let t2096 = F::cast_from(0.49234783862717707_f64) * t1765;
    let t2098 = F::cast_from(0.2946275542389858_f64) * t1684;
    let t2100 = F::cast_from(0.0982091847463286_f64) * t1735;
    let t2102 = t2094 - F::cast_from(1.4770435158815312_f64) * t1762 + t2096 + F::cast_from(1.4770435158815312_f64) * t1769 + t2098 - F::cast_from(0.2946275542389858_f64) * t1732 + t2100 + F::cast_from(0.2946275542389858_f64) * t1738;
    let t2103 = t447 * t2102;
    let t2104 = t2103 * t452;
    (t2091, t2094, t2096, t2098, t2100, t2102, t2103, t2104)
}
