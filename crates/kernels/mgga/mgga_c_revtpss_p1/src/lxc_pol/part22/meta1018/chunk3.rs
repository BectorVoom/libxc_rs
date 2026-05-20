//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3524/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3524<F: Float>(t11672: F, t11883: F, t15725: F, t15830: F, t16226: F, t1675: F, t19873: F, t20083: F, t42215: F, t4831: F, t54546: F, t54550: F, t54553: F, t55356: F, t6289: F, t66128: F, t66721: F, t66731: F, t66734: F, t66739: F, t66747: F) -> F {
    let t66749 = -t66721 / F::new(1296.0) + F::new(11.0) / F::new(324.0) * t11883 * t6289 + F::cast_from(0.31758531939310916275e-3_f64) * t54546 + F::cast_from(0.3811023832717309953e-3_f64) * t54550 + F::cast_from(0.3811023832717309953e-3_f64) * t54553 + F::cast_from(0.30488190661738479624e-2_f64) * t11672 * t19873 - F::cast_from(0.3811023832717309953e-3_f64) * t66731 + F::cast_from(0.19055119163586549765e-2_f64) * t16226 * t66734 * t42215 * t66128 + F::cast_from(0.19055119163586549765e-3_f64) * t66739 + F::cast_from(0.85748036236139473944e-3_f64) * t15725 * t20083 - F::cast_from(0.30488190661738479624e-2_f64) * t55356 * t1675 - F::cast_from(0.30488190661738479624e-2_f64) * t15830 * t4831 + F::cast_from(0.3811023832717309953e-3_f64) * t66747;
    t66749
}
