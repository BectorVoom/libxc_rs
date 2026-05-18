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
    let t10674 = -F::new(35.0) / F::new(72.0) * t6933 - F::new(0.24009450146119052704e-1) * t8946 + F::new(0.12862205435420921092e-1) * t612 * t10643 - F::new(0.12862205435420921092e-2) * t5390 * t10647 - F::new(0.51448821741683684367e-2) * t5244 * t10651 + F::new(0.25724410870841842183e-2) * t1733 * t10655 - F::new(0.12862205435420921092e-1) * t5279 * t10659 - t5289 - F::new(0.60023625365297631762e-1) * t8991 + F::new(0.12004725073059526352e-1) * t9008 - t5405 - F::new(0.68026775414003982663e-1) * t6988 - F::new(0.25724410870841842183e-1) * t612 * t10666 - F::new(0.85748036236139473944e-3) * t612 * t10670 - F::new(0.17006693853500995666e-1) * t6995;
    (t10666, t10670, t10674)
}
