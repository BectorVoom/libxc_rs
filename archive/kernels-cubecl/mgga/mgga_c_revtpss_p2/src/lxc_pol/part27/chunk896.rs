//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 896/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk896<F: Float>(t10818: F, t2477: F, t828: F, t222: F, t9727: F, t2737: F, t9802: F, t10639: F, t827: F, t221: F, t2485: F, t2754: F) -> (F, F, F, F, F) {
    let t10820 = t2477 * t828 * t10818;
    let t10824 = F::cast_from(455.0_f64) / F::cast_from(1296.0_f64) * t9727 * t222;
    let t10826 = F::cast_from(0.45738002528356795401e-4_f64) * t9802 * t2737;
    let t10828 = t827 * t828 * t10639;
    let t10832 = t2485 * t221 * t2754;
    (t10820, t10824, t10826, t10828, t10832)
}
