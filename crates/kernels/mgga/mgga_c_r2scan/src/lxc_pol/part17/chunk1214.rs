//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1214/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1214<F: Float>(t11338: F, t12422: F, t11518: F, t12098: F, t3262: F, t11345: F, t12567: F, t11523: F, t12086: F, t11199: F, t12570: F, t3275: F, t3472: F, t42901: F) -> (F, F, F, F, F, F) {
    let t44122 = t12422 * t11338 / F::new(4.0);
    let t44125 = F::new(15.0) / F::new(8.0) * t3262 * t12098 * t11518;
    let t44127 = t12567 * t11345 / F::new(4.0);
    let t44129 = t11523 * t12086 / F::new(2.0);
    let t44132 = F::new(3.0) / F::new(4.0) * t3262 * t11199 * t12570;
    let t44135 = F::new(5.0) / F::new(16.0) * t3275 * t3472 * t42901;
    (t44122, t44125, t44127, t44129, t44132, t44135)
}
