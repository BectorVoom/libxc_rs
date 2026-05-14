//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 643/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk643<F: Float>(t10772: F, t600: F, t10692: F, t10700: F, t10707: F, t10709: F, t10712: F, t10718: F, t10752: F, t10760: F, t1674: F, t45: F, t4757: F, t4764: F, t4791: F, t10686: F) -> (F, F) {
    let t10773 = t10772 * t600;
    let t10776 = -0.51947267698127589899e2 * t4757 * t4791 + 0.1038945353962551798e3 * t1674 * t10692 - 0.1025389702100779493e4 * t1674 * t10700 + 0.35089340384731224426e1 * t4757 * t4764 + t10707 + t10709 + t10712 - t10718 + t10752 + t10760 + 0.19751789702565206229e-1 * t45 * t10773;
    let t10777 = t10686 + t10776;
    (t10773, t10777)
}
