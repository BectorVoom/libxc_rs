//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 945/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk945<F: Float>(t2681: F, t820: F, t823: F, t222: F, t9727: F, t2737: F, t9802: F, t2482: F, t596: F, t27: F, t2719: F, t843: F) -> (F, F, F, F, F, F) {
    let t10815 = t820 * t823 * t2681;
    let t10824 = F::cast_from(455.0_f64) / F::cast_from(1296.0_f64) * t9727 * t222;
    let t10826 = F::cast_from(0.45738002528356795401e-4_f64) * t9802 * t2737;
    let t10845 = t2482 * t823 * t596;
    let t10850 = t2482 * t2719 * t27;
    let t10858 = t820 * t2719 * t843;
    (t10815, t10824, t10826, t10845, t10850, t10858)
}
