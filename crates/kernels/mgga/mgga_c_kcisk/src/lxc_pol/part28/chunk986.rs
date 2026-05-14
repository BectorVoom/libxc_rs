//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 986/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk986<F: Float>(t1674: F, t22816: F, t22820: F, t22870: F, t22872: F, t22888: F, t22893: F, t22895: F, t22897: F, t22899: F, t22901: F, t22908: F, t22915: F, t45: F, t4757: F, t6851: F, t6857: F, t6876: F, t6881: F, t8592: F, t8613: F) -> (F,) {
    let t22918 = 0.1038945353962551798e3 * t1674 * t22816 + 0.11696446794910408142e1 * t1674 * t22820 + t22870 + t22872 + 0.19751789702565206229e-1 * t45 * t22888 + t22893 + t22895 + t22897 - t22899 + t22901 - 0.11696446794910408142e1 * t6851 * t6876 + 0.11696446794910408142e1 * t4757 * t8592 - 0.346315117987517266e2 * t6851 * t6881 - 0.35089340384731224426e1 * t1674 * t22908 + 0.23392893589820816284e1 * t6851 * t6857 - 0.17315755899375863299e2 * t4757 * t8613 + 0.23392893589820816284e1 * t1674 * t22915;
    (t22918,)
}
