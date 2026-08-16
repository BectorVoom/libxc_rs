//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1220/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1220<F: Float>(t5478: F, t5473: F, t113: F, t1273: F, t1275: F, t18798: F, t19920: F, t21805: F, t21818: F, t21898: F, t21900: F, t332: F, t4322: F, t4381: F, t4635: F, t5474: F, t5475: F, t5479: F, t5480: F, t5483: F, t82074: F, t889: F) -> F {
    let t91437 = t5478 * t5478;
    let t91446 = t5473 * t5473;
    let t91469 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t19920 * t5475 + F::cast_from(3.0_f64) * t19920 * t5483 + t889 * t91437 * t332 * t113 / F::cast_from(4.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t889 * t5479 * t113 * t5473 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t889 * t91446 * t332 * t113 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t19920 * t5480 + t889 * t21898 * t1273 * t4381 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t889 * t5474 * t4635 + F::cast_from(3.0_f64) * t889 * t21805 * t18798 + t4322 * t21900 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t889 * t5479 * t4635 + t82074 * t1275 + F::cast_from(3.0_f64) * t4322 * t21818;
    t91469
}
