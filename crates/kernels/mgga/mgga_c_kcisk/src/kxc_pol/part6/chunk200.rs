//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 200/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk200<F: Float>(t782: F, t788: F, t722: F, t772: F, t737: F, t749: F, t240: F, t753: F, t567: F, t157: F, t32: F, t5: F) -> (F, F, F, F, F, F, F, F, F) {
    let t791 = F::new(1.0) + F::cast_from(0.2698618307426597582e-1_f64) * t782 * t788;
    let t792 = F::ln(t791);
    let t794 = F::new(1.0) + F::new(0.193e0) * t792;
    let t795 = F::new(1.0) / t794;
    let t798 = t772 * t795 + F::cast_from(0.17411041666666666666e-2_f64) * t722;
    let t801 = F::new(1.0) + F::new(0.9375e-1) * t737 - F::cast_from(0.101171875e-1_f64) * t749;
    let t802 = F::new(1.0) / t801;
    let t806 = t753 + t240 * (t798 * t802 - t753);
    let t807 = t567 * t806;
    let t812 = F::cast_from(0.11073577833333333333e-2_f64) * t5 * t157 * t32;
    (t791, t794, t795, t798, t801, t802, t806, t807, t812)
}
