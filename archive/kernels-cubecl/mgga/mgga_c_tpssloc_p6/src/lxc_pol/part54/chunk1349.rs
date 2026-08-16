//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1349/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1349<F: Float>(t1985: F, t26471: F, t6889: F, t6906: F, t1388: F, t7752: F, t1307: F, t26179: F, t8327: F, t31058: F, t7458: F, t19456: F) -> (F, F, F, F, F, F) {
    let t120649 = F::cast_from(0.16449340668482264365e-1_f64) * t1985 * t6889 * t6906 * t26471;
    let t120694 = t7752 * t1388;
    let t120705 = t7752 * t1307;
    let t120719 = F::cast_from(2.0_f64) * t26179 * t8327;
    let t120721 = F::cast_from(2.0_f64) * t7458 * t31058;
    let t120728 = F::cast_from(2.0_f64) * t19456 * t8327;
    (t120649, t120694, t120705, t120719, t120721, t120728)
}
