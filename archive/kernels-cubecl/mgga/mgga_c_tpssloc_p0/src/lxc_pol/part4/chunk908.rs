//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 908/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk908<F: Float>(t11717: F, t3503: F, t11713: F, t11708: F, t3514: F, t1210: F, t3247: F, t415: F, t121: F, t3584: F, t1229: F, t676: F) -> (F, F, F, F, F, F) {
    let t11727 = t3503 * t11717;
    let t11728 = t11713 * t11727;
    let t11734 = t11708 * t3514;
    let t11737 = t1210 * t11717;
    let t11738 = t11713 * t11737;
    let t11778 = F::cast_from(1.0_f64) / t415 / t3247;
    let t11784 = t121 * t3584;
    let t11789 = t676 * t1229;
    (t11728, t11734, t11738, t11778, t11784, t11789)
}
