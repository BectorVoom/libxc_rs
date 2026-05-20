//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1993/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1993<F: Float>(t1238: F, t17296: F, t17298: F, t17301: F, t17304: F, t17337: F, t17609: F, t1797: F, t20838: F, t20843: F, t20847: F, t20851: F, t5274: F, t5287: F, t5293: F, t5331: F) -> F {
    let t20855 = -F::cast_from(0.22866142996303859718e-2_f64) * t5293 * t5287 + F::cast_from(0.42874018118069736972e-3_f64) * t17609 * t1797 + F::cast_from(0.42874018118069736972e-3_f64) * t5274 * t5287 - F::cast_from(0.42874018118069736972e-3_f64) * t5331 * t20838 - F::cast_from(0.14291339372689912324e-3_f64) * t20843 + F::cast_from(0.28582678745379824648e-3_f64) * t20847 - F::cast_from(0.21437009059034868486e-3_f64) * t20851 * t1238 - t17296 + t17298 - t17301 + F::cast_from(0.95275595817932748827e-4_f64) * t17304 - t17337;
    t20855
}
