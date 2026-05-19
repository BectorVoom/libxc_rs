//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 833/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk833<F: Float>(t810: F, t9784: F, t9789: F, t235: F, t2783: F, t2453: F, t2475: F, t72: F, t245: F, t2482: F, t814: F, t823: F) -> (F, F, F, F, F) {
    let t10756 = F::cast_from(0.72250660161932334527e-3_f64) * t9784 * t810;
    let t10758 = F::cast_from(0.11294745624363664198e-6_f64) * t9789 * t810;
    let t10759 = t2783 * t235;
    let t10760 = t2453 * t10759;
    let t10769 = t2475 * t72;
    let t10770 = t10769 * t245;
    let t10777 = t2482 * t823 * t814;
    (t10756, t10758, t10760, t10770, t10777)
}
