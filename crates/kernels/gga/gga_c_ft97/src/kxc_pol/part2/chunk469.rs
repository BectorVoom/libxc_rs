//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 469/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk469<F: Float>(t299: F, t2961: F, t332: F, t113: F, t505: F, t910: F, t1934: F, t2900: F, t2904: F, t2958: F, t333: F, t5: F, t886: F, t889: F, t911: F, t1537: F, t947: F) -> (F, F, F, F, F) {
    let t300 = 10000000.0 <= t299;
    let t2962 = t2961 * t332;
    let t2963 = t2962 * t113;
    let t2966 = t910 * t505;
    let t2973 = piecewise3(t300, 0.0, t5 * t2900 * t113 / 4.0 + t2904 * t911 / 2.0 + t5 * t886 * t505 / 2.0 + t889 * t2958 / 4.0 + t889 * t2963 / 4.0 + t889 * t2966 / 2.0 + t5 * t333 * t1934 / 4.0);
    let t2976 = t1537 * t947;
    (t2962, t2963, t2966, t2973, t2976)
}
