//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 974/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk974<F: Float>(t265: F, t502: F, t29154: F, t29210: F, t29258: F, t29311: F, t3801: F, t8220: F, t1298: F, t1832: F, t1300: F, t198: F, t27037: F, t27041: F, t27754: F, t336: F, t5023: F, t5501: F, t7673: F) -> F {
    let t503 = t265 < t502;
    let t29313 = t29154 + t29210 + t29258 + t29311;
    let t29317 = t8220 * t3801;
    let t29322 = t1832 * t1298;
    let t29329 = piecewise3::<f64>(t503, t1300 * t198 * t29313 * t336 - t1298 * t29317 * t5023 - t1832 * t27037 * t5023 + F::new(2.0) * t27041 * t29322 * t5023 - t5023 * t5501 * t7673, t27754);
    t29329
}
