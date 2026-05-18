//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 472/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk472<F: Float>(t2380: F, t2697: F, t274: F, t2417: F, t801: F, t231: F, t123: F, t194: F, t805: F, t278: F, t2014: F, t2394: F, t807: F) -> (F, F, F, F, F) {
    let t2698 = t2697 * t2380;
    let t2699 = t2698 * t274;
    let t2701 = t801 * t2417;
    let t2702 = t2701 * t274;
    let t2704 = t2380 * t274;
    let t2705 = t231 * t2704;
    let t2710 = t123 / t805 / t194;
    let t2711 = t2380 * t278;
    let t2719 = -F::new(0.11705142615505742e0) * t2699 + F::new(0.23410285231011484e0) * t2702 - F::new(0.26564305359272358183e-2) * t2014 * t2705 + F::new(0.319782988780431561e-1) * t2710 * t2711 - F::new(0.532971647967385935e-1) * t807 * t2417 * t278 + F::new(0.13977476158628290272e-1) * t2394 * t2711;
    (t2698, t2701, t2704, t2710, t2719)
}
