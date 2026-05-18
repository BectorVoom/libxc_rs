//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 939/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk939<F: Float>(t14483: F, t14560: F, t332: F, t113: F, t10829: F, t1275: F, t14391: F, t14395: F, t14403: F, t14409: F, t14412: F, t2904: F, t2963: F, t4322: F, t4377: F, t4382: F, t4385: F, t4391: F, t4395: F, t889: F) -> F {
    let t14561 = t14483 + t14560;
    let t14562 = t14561 * t332;
    let t14563 = t14562 * t113;
    let t14568 = t889 * t14391 / F::new(4.0) + t889 * t14395 / F::new(4.0) + t2904 * t4377 / F::new(2.0) + t2904 * t4385 / F::new(2.0) + t889 * t14403 / F::new(2.0) + t2904 * t4382 / F::new(2.0) + t889 * t14409 / F::new(2.0) + t889 * t14412 / F::new(4.0) + t2904 * t4391 / F::new(2.0) - t2904 * t4395 + t10829 * t1275 / F::new(4.0) + t889 * t14563 / F::new(4.0) + t4322 * t2963 / F::new(4.0);
    t14568
}
