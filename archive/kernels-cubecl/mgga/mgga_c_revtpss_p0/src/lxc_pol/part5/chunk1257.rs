//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1257/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1257<F: Float>(t11947: F, t15745: F, t16134: F, t16160: F, t16190: F, t1665: F, t1671: F, t20017: F, t20021: F, t20025: F, t20030: F, t20034: F, t3188: F, t6327: F, t6339: F) -> F {
    let t20036 = -F::cast_from(0.22866142996303859718e-2_f64) * t11947 * t6339 + F::cast_from(0.28582678745379824648e-3_f64) * t20017 - F::cast_from(0.14291339372689912324e-3_f64) * t20021 + F::cast_from(0.22866142996303859718e-2_f64) * t15745 * t1665 - F::cast_from(0.28582678745379824648e-3_f64) * t20025 + t16134 + F::cast_from(0.23818898954483187207e-3_f64) * t3188 * t6327 + F::cast_from(0.28582678745379824648e-3_f64) * t20030 - F::cast_from(0.22866142996303859718e-2_f64) * t16190 * t1671 + F::cast_from(0.28582678745379824648e-3_f64) * t20034 + t16160;
    t20036
}
