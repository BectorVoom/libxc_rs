//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1658/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1658<F: Float>(t562: F, t6414: F, t5250: F, t12171: F, t6388: F, t3901: F, t6415: F, t11984: F, t15880: F, t15889: F, t15894: F, t19543: F, t19574: F, t19576: F, t19581: F, t19588: F, t19589: F, t19590: F, t19592: F, t19594: F, t9457: F, t9476: F, t9484: F) -> (F, F, F, F, F) {
    let t19660 = t562 * t6414;
    let t19661 = t19660 * t5250;
    let t19668 = t12171 * t6388;
    let t19674 = t3901 * t6415;
    let t19676 = -t19543 - t9457 + t19574 + t19576 + t9476 + t9484 - t19581 - t15880 + t19588 + t15889 - t19589 - t15894 - t19590 - t11984 + t19592 - t19594;
    (t19660, t19661, t19668, t19674, t19676)
}
