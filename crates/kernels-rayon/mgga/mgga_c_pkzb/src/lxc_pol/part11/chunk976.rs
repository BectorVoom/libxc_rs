//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 976/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk976(t10502: f64, t5305: f64, t616: f64, t10556: f64, t615: f64, t10643: f64, t10647: f64, t10651: f64, t10655: f64, t10659: f64, t1733: f64, t5244: f64, t5279: f64, t5289: f64, t5390: f64, t5405: f64, t612: f64, t6933: f64, t6988: f64, t6995: f64, t8946: f64, t8991: f64, t9008: f64) -> (f64, f64, f64) {
    let t10666 = t5305 * t616 * t10502;
    let t10670 = t615 * t616 * t10556;
    let t10674 = -35.0_f64 / 72.0_f64 * t6933 - 0.24009450146119052704e-1_f64 * t8946 + 0.12862205435420921092e-1_f64 * t612 * t10643 - 0.12862205435420921092e-2_f64 * t5390 * t10647 - 0.51448821741683684367e-2_f64 * t5244 * t10651 + 0.25724410870841842183e-2_f64 * t1733 * t10655 - 0.12862205435420921092e-1_f64 * t5279 * t10659 - t5289 - 0.60023625365297631762e-1_f64 * t8991 + 0.12004725073059526352e-1_f64 * t9008 - t5405 - 0.68026775414003982663e-1_f64 * t6988 - 0.25724410870841842183e-1_f64 * t612 * t10666 - 0.85748036236139473944e-3_f64 * t612 * t10670 - 0.17006693853500995666e-1_f64 * t6995;
    (t10666, t10670, t10674)
}
