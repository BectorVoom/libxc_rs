//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 690/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk690(t10772: f64, t600: f64, t10692: f64, t10700: f64, t10707: f64, t10709: f64, t10712: f64, t10718: f64, t10752: f64, t10760: f64, t1674: f64, t45: f64, t4757: f64, t4764: f64, t4791: f64) -> (f64, f64) {
    let t10773 = t10772 * t600;
    let t10776 = -0.51947267698127589899e2_f64 * t4757 * t4791 + 0.1038945353962551798e3_f64 * t1674 * t10692 - 0.1025389702100779493e4_f64 * t1674 * t10700 + 0.35089340384731224426e1_f64 * t4757 * t4764 + t10707 + t10709 + t10712 - t10718 + t10752 + t10760 + 0.19751789702565206229e-1_f64 * t45 * t10773;
    (t10773, t10776)
}
