//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 899/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk899<F: Float>(t33: F, t265: F, t502: F, t34943: F, t34994: F, t1300: F, t1832: F, t198: F, t33533: F, t33539: F, t336: F, t33866: F, t5023: F, t7673: F, t8220: F, t1469: F, t33896: F, t57: F, t8960: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t34995 = t34943 + t34994;
    let t35008 = piecewise3(t503, t1300 * t198 * t336 * t34995 - t1832 * t33533 * t5023 + 2.0 * t1832 * t33539 * t5023 - 2.0 * t5023 * t7673 * t8220, t33866);
    let t35013 = piecewise3(t400, t33896, -t8960 * t1469 / 2.0 + t35008 * t57 / 2.0);
    (t34995, t35008, t35013)
}
