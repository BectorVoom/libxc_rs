//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1192/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1192(t15971: f64, t592: f64, t2221: f64, t5168: f64, t2225: f64, t5154: f64, t9892: f64, t9722: f64, t1788: f64, t9216: f64, t9218: f64, t9494: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t54412 = t592 * t15971;
    let t54428 = t2221 * t5168;
    let t54432 = t2225 * t5168;
    let t54434 = t5154 * t9892;
    let t54451 = t5154 * t9722;
    let t54460 = t9216 * t1788;
    let t54462 = t9218 * t1788;
    let t54467 = t5154 * t9494;
    (t54412, t54428, t54432, t54434, t54451, t54460, t54462, t54467)
}
