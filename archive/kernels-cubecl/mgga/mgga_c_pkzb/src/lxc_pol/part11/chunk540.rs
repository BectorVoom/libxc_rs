//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 540/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk540<F: Float>(t2852: F, t721: F, t1096: F, t1108: F, t1911: F, t1916: F, t1938: F, t1950: F, t1955: F, t1977: F, t248: F, t2745: F, t2748: F, t2750: F, t2753: F, t2785: F, t2789: F, t2793: F, t2796: F, t2801: F, t2816: F, t2820: F, t2827: F, t2829: F, t2834: F, t2849: F, t695: F, t704: F, t714: F, t723: F) -> (F, F) {
    let t2853 = t2852 * t721;
    let t2856 = -F::cast_from(0.310907e-1_f64) * t2793 * t248 + F::cast_from(1.0_f64) * t2796 * t704 + F::cast_from(1.0_f64) * t1911 * t1096 - F::cast_from(2.0_f64) * t1916 * t2801 + F::cast_from(1.0_f64) * t695 * t2816 + F::cast_from(0.32163958997385070134e2_f64) * t1938 * t2820 + t2745 - t2748 - t2750 + t2753 - t2785 - t2789 - F::cast_from(0.19751673498613801407e-1_f64) * t2827 + F::cast_from(0.5848223622634646207e0_f64) * t2829 * t723 + F::cast_from(0.5848223622634646207e0_f64) * t1950 * t1108 - F::cast_from(0.11696447245269292414e1_f64) * t1955 * t2834 + F::cast_from(0.5848223622634646207e0_f64) * t714 * t2849 + F::cast_from(0.17315859105681463759e2_f64) * t1977 * t2853;
    (t2853, t2856)
}
