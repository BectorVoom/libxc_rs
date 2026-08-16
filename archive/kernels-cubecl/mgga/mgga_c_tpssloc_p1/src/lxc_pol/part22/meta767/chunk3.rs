//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2595/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2595<F: Float>(t18321: F, t5040: F, t1174: F, t1177: F, t1748: F, t19002: F, t19005: F, t19047: F, t19051: F, t4889: F, t5014: F, t5030: F, t52628: F, t65581: F, t65598: F, t65600: F, t65605: F, t65607: F, t65613: F, t71168: F, t71177: F) -> F {
    let t72352 = t18321 * t5040;
    let t72357 = t4889 * t19005 / F::cast_from(6.0_f64) - t1174 * t1177 * t71177 / F::cast_from(144.0_f64) - t1174 * t1177 * t71168 / F::cast_from(16.0_f64) - t65581 / F::cast_from(4608.0_f64) + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t65598 + t52628 * t19002 / F::cast_from(72.0_f64) + t19047 * t5014 / F::cast_from(1024.0_f64) - t65607 * t1748 / F::cast_from(1536.0_f64) - t19051 * t5030 / F::cast_from(1536.0_f64) - F::cast_from(11.0_f64) / F::cast_from(324.0_f64) * t72352 + t65600 / F::cast_from(432.0_f64) - t65605 / F::cast_from(2304.0_f64) - t65613 / F::cast_from(1152.0_f64);
    t72357
}
