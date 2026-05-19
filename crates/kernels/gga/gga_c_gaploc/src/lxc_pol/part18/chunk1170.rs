//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1170/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1170<F: Float>(t31590: F, t6508: F, t1358: F, t6507: F, t10269: F, t3808: F, t29896: F, t29898: F, t29901: F, t29903: F, t29908: F, t29911: F, t29913: F, t29915: F, t471: F) -> (F, F, F, F) {
    let t31591 = t6508 * t31590;
    let t31594 = F::cast_from(0.12646669615856066488e-1_f64) * t1358 * t6507 * t31591;
    let t31600 = F::cast_from(0.12646669615856066488e-1_f64) * t3808 * t10269;
    let t31610 = (F::new(189.0) / F::new(512.0) * t29896 - F::new(2499.0) / F::new(16384.0) * t29898 + F::new(1239.0) / F::new(524288.0) * t29901 - F::new(441.0) / F::new(0.16777216e8) * t29903 + F::new(147.0) / F::new(0.16777216e8) * t29908 - F::new(413.0) / F::new(524288.0) * t29911 + F::new(833.0) / F::new(16384.0) * t29913 - F::new(63.0) / F::new(512.0) * t29915) * t471;
    (t31591, t31594, t31600, t31610)
}
