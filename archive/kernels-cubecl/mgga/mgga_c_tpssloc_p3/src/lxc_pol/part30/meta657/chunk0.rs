//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2075/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2075<F: Float>(t90470: F, t26356: F, t6914: F, t1799: F, t3886: F, t1887: F, t80827: F, t26334: F, t26339: F, t81159: F, t22716: F, t7697: F) -> (F, F, F, F, F, F, F) {
    let t90471 = F::cast_from(0.76763589786250567036e-1_f64) * t90470;
    let t90472 = t6914 * t26356;
    let t90473 = F::cast_from(0.76763589786250567036e-1_f64) * t90472;
    let t90488 = t3886 * t1799;
    let t90497 = t80827 * t1887;
    let t90498 = t90497 * t26334;
    let t90500 = t81159 * t26339;
    let t90501 = F::cast_from(0.76763589786250567036e-1_f64) * t90500;
    let t90503 = t22716 * t7697;
    (t90471, t90473, t90488, t90497, t90498, t90501, t90503)
}
