//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1029/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1029(t19885: f64, t19898: f64, t332: f64, t113: f64, t18795: f64, t18799: f64, t18802: f64, t18804: f64, t18809: f64, t18812: f64, t18946: f64, t18953: f64, t2904: f64, t4322: f64, t4391: f64, t4395: f64, t5: f64, t5483: f64, t889: f64) -> f64 {
    let t19899 = t19885 + t19898;
    let t19900 = t19899 * t332;
    let t19904 = t889 * t18795 / 2.0_f64 + t889 * t18799 / 2.0_f64 - t889 * t18802 + t889 * t18804 / 4.0_f64 + t2904 * t5483 / 2.0_f64 + t889 * t18809 / 2.0_f64 + t889 * t18812 / 4.0_f64 + t889 * t18946 / 4.0_f64 + t4322 * t4391 / 2.0_f64 - t4322 * t4395 + t889 * t18953 / 4.0_f64 + t5 * t19900 * t113 / 4.0_f64;
    t19904
}
