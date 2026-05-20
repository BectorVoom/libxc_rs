//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1174/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1174<F: Float>(t3889: F, t5651: F, t13716: F, t1394: F, t13892: F, t13902: F, t13907: F, t13911: F, t1392: F, t1395: F, t1877: F, t1879: F, t4045: F, t4050: F, t4053: F, t539: F, t541: F, t5644: F, t5650: F, t5652: F, t5655: F) -> F {
    let t13914 = t5651 * t3889;
    let t13917 = t1394 * t13716;
    let t13920 = -t13892 * t541 - F::new(24.0) * t13902 * t5652 + F::new(60.0) * t13907 * t5650 - F::new(24.0) * t13911 * t5650 - F::new(12.0) * t13914 * t5650 + F::new(3.0) * t13917 * t539 + F::new(6.0) * t1392 * t5655 + F::new(6.0) * t1395 * t5644 - F::new(12.0) * t1877 * t4050 + F::new(3.0) * t1877 * t4053 + F::new(3.0) * t1879 * t4045;
    t13920
}
