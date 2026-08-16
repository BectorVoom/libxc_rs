//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1220/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1220(t5478: f64, t5473: f64, t113: f64, t1273: f64, t1275: f64, t18798: f64, t19920: f64, t21805: f64, t21818: f64, t21898: f64, t21900: f64, t332: f64, t4322: f64, t4381: f64, t4635: f64, t5474: f64, t5475: f64, t5479: f64, t5480: f64, t5483: f64, t82074: f64, t889: f64) -> f64 {
    let t91437 = t5478 * t5478;
    let t91446 = t5473 * t5473;
    let t91469 = 3.0_f64 / 2.0_f64 * t19920 * t5475 + 3.0_f64 * t19920 * t5483 + t889 * t91437 * t332 * t113 / 4.0_f64 + 3.0_f64 / 2.0_f64 * t889 * t5479 * t113 * t5473 + 3.0_f64 / 4.0_f64 * t889 * t91446 * t332 * t113 + 3.0_f64 / 2.0_f64 * t19920 * t5480 + t889 * t21898 * t1273 * t4381 + 3.0_f64 / 2.0_f64 * t889 * t5474 * t4635 + 3.0_f64 * t889 * t21805 * t18798 + t4322 * t21900 + 3.0_f64 / 2.0_f64 * t889 * t5479 * t4635 + t82074 * t1275 + 3.0_f64 * t4322 * t21818;
    t91469
}
