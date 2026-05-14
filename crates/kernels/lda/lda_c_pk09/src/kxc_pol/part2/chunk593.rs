//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 593/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk593<F: Float>(t309: F, t5420: F, t1339: F, t1519: F, t318: F, t5308: F, t317: F, t337: F, t280: F, t1632: F, t1625: F, t4764: F, t5039: F, t5045: F, t5068: F, t4944: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5680 = t5420 * t309;
    let t5681 = t5680 * t1339;
    let t5683 = t318 * t1519;
    let t5684 = t5683 * t5308;
    let t5686 = t317 * t337;
    let t5687 = t5686 * t280;
    let t5688 = t1632 * t5687;
    let t5689 = t5688 * t1625;
    let t5691 = 0.06655833038988691 * t4764;
    let t5693 = 0.10237773105191754 * t5039;
    let t5694 = 0.06825182070127836 * t5045;
    let t5696 = 0.02275060690042612 * t5068;
    let t5701 = 0.04933718966136796 * t4944;
    (t5681, t5683, t5684, t5687, t5689, t5691, t5693, t5694, t5696, t5701)
}
