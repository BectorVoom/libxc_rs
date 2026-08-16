//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 976/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk976<F: Float>(t10502: F, t5305: F, t616: F, t10556: F, t615: F, t10643: F, t10647: F, t10651: F, t10655: F, t10659: F, t1733: F, t5244: F, t5279: F, t5289: F, t5390: F, t5405: F, t612: F, t6933: F, t6988: F, t6995: F, t8946: F, t8991: F, t9008: F) -> (F, F, F) {
    let t10666 = t5305 * t616 * t10502;
    let t10670 = t615 * t616 * t10556;
    let t10674 = -F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t6933 - F::cast_from(0.24009450146119052704e-1_f64) * t8946 + F::cast_from(0.12862205435420921092e-1_f64) * t612 * t10643 - F::cast_from(0.12862205435420921092e-2_f64) * t5390 * t10647 - F::cast_from(0.51448821741683684367e-2_f64) * t5244 * t10651 + F::cast_from(0.25724410870841842183e-2_f64) * t1733 * t10655 - F::cast_from(0.12862205435420921092e-1_f64) * t5279 * t10659 - t5289 - F::cast_from(0.60023625365297631762e-1_f64) * t8991 + F::cast_from(0.12004725073059526352e-1_f64) * t9008 - t5405 - F::cast_from(0.68026775414003982663e-1_f64) * t6988 - F::cast_from(0.25724410870841842183e-1_f64) * t612 * t10666 - F::cast_from(0.85748036236139473944e-3_f64) * t612 * t10670 - F::cast_from(0.17006693853500995666e-1_f64) * t6995;
    (t10666, t10670, t10674)
}
