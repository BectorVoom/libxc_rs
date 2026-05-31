//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1016/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1016<F: Float>(t265: F, t502: F, t34943: F, t34994: F, t1300: F, t1832: F, t198: F, t33533: F, t33539: F, t336: F, t33866: F, t5023: F, t7673: F, t8220: F) -> (F, F) {
    let t503 = t265 < t502;
    let t34995 = t34943 + t34994;
    let t35008 = piecewise3::<F>(t503, t1300 * t198 * t336 * t34995 - t1832 * t33533 * t5023 + F::cast_from(2.0_f64) * t1832 * t33539 * t5023 - F::cast_from(2.0_f64) * t5023 * t7673 * t8220, t33866);
    (t34995, t35008)
}
