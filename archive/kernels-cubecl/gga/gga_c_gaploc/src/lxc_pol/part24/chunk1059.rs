//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1059/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1059<F: Float>(t16922: F, t278: F, t481: F, t16889: F, t2547: F, t686: F, t1710: F, t935: F, t7290: F, t296: F, t7112: F, t830: F) -> (F, F, F, F, F, F, F) {
    let t21571 = t481 * t16922 * t278;
    let t21636 = t481 * t16889 * t278;
    let t21665 = t481 * t2547 * t686;
    let t21783 = t935 * t1710;
    let t21784 = t7290 * t21783;
    let t21794 = t296 * t7112;
    let t21888 = t830 * t935;
    (t21571, t21636, t21665, t21783, t21784, t21794, t21888)
}
