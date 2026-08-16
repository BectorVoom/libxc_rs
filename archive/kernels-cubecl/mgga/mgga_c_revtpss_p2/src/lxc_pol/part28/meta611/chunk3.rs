//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2136/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2136<F: Float>(t27123: F, t7003: F, t13514: F, t94: F, t1937: F, t27126: F, t6993: F, t25178: F, t7898: F, t22496: F, t25082: F, t32113: F) -> (F, F, F, F, F) {
    let t98534 = F::cast_from(4.0_f64) * t27123 * t7003;
    let t98535 = t94 * t13514;
    let t98537 = F::cast_from(2.0_f64) * t98535 * t1937;
    let t98539 = F::cast_from(4.0_f64) * t27126 * t6993;
    let t98541 = F::cast_from(2.0_f64) * t7898 * t25178;
    let t98544 = F::cast_from(6.0_f64) * t25082 * t32113 * t22496;
    (t98534, t98537, t98539, t98541, t98544)
}
