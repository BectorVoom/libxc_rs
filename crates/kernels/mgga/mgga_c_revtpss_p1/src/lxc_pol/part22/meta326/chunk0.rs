//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1776/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1776<F: Float>(t10815: F, t839: F, t222: F, t9727: F, t2737: F, t9802: F, t221: F, t2485: F, t2754: F, t2484: F, t2749: F, t836: F, t853: F) -> (F, F, F, F, F, F) {
    let t10816 = t10815 * t839;
    let t10824 = F::new(455.0) / F::new(1296.0) * t9727 * t222;
    let t10826 = F::cast_from(0.45738002528356795401e-4_f64) * t9802 * t2737;
    let t10832 = t2485 * t221 * t2754;
    let t10833 = t2484 * t10832;
    let t10836 = t853 * t836 * t2749;
    (t10816, t10824, t10826, t10832, t10833, t10836)
}
