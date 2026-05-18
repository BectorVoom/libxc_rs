//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 980/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk980<F: Float>(t265: F, t502: F, t33467: F, t33528: F, t3801: F, t8951: F, t12587: F, t8955: F, t1298: F, t1300: F, t198: F, t32058: F, t336: F, t5023: F, t7669: F, t7673: F) -> (F, F, F, F) {
    let t503 = t265 < t502;
    let t33529 = t33467 + t33528;
    let t33533 = t8951 * t3801;
    let t33539 = t8955 * t12587;
    let t33544 = piecewise3::<f64>(t503, t1300 * t198 * t33529 * t336 - t1298 * t33533 * t5023 + F::new(2.0) * t1298 * t33539 * t5023 - F::new(2.0) * t5023 * t7669 * t7673, t32058);
    (t33529, t33533, t33539, t33544)
}
