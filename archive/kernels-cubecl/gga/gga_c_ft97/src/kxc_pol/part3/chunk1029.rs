//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1029/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1029<F: Float>(t19885: F, t19898: F, t332: F, t113: F, t18795: F, t18799: F, t18802: F, t18804: F, t18809: F, t18812: F, t18946: F, t18953: F, t2904: F, t4322: F, t4391: F, t4395: F, t5: F, t5483: F, t889: F) -> F {
    let t19899 = t19885 + t19898;
    let t19900 = t19899 * t332;
    let t19904 = t889 * t18795 / F::cast_from(2.0_f64) + t889 * t18799 / F::cast_from(2.0_f64) - t889 * t18802 + t889 * t18804 / F::cast_from(4.0_f64) + t2904 * t5483 / F::cast_from(2.0_f64) + t889 * t18809 / F::cast_from(2.0_f64) + t889 * t18812 / F::cast_from(4.0_f64) + t889 * t18946 / F::cast_from(4.0_f64) + t4322 * t4391 / F::cast_from(2.0_f64) - t4322 * t4395 + t889 * t18953 / F::cast_from(4.0_f64) + t5 * t19900 * t113 / F::cast_from(4.0_f64);
    t19904
}
