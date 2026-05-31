//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 500/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk500<F: Float>(t299: F, t2956: F, t332: F, t113: F, t909: F, t505: F, t910: F, t1934: F, t2900: F, t2904: F, t333: F, t5: F, t886: F, t889: F, t911: F) -> (F, F, F, F, F, F, F) {
    let t300 = F::cast_from(10000000.0_f64) <= t299;
    let t2957 = t2956 * t332;
    let t2958 = t2957 * t113;
    let t2961 = t909 * t909;
    let t2962 = t2961 * t332;
    let t2963 = t2962 * t113;
    let t2966 = t910 * t505;
    let t2973 = piecewise3::<F>(t300, F::cast_from(0.0_f64), t5 * t2900 * t113 / F::cast_from(4.0_f64) + t2904 * t911 / F::cast_from(2.0_f64) + t5 * t886 * t505 / F::cast_from(2.0_f64) + t889 * t2958 / F::cast_from(4.0_f64) + t889 * t2963 / F::cast_from(4.0_f64) + t889 * t2966 / F::cast_from(2.0_f64) + t5 * t333 * t1934 / F::cast_from(4.0_f64));
    (t2957, t2958, t2961, t2962, t2963, t2966, t2973)
}
