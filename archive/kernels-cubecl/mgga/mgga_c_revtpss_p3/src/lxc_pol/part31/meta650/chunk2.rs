//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2146/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2146<F: Float>(t19882: F, t7132: F, t27450: F, t4820: F, t100024: F, t100048: F, t100051: F, t100078: F, t19838: F, t20105: F, t25577: F, t25580: F, t6273: F, t6331: F, t93543: F, t93555: F) -> F {
    let t106923 = t7132 * t19882;
    let t106926 = t27450 * t4820;
    let t106929 = -F::cast_from(0.42874018118069736972e-3_f64) * t25580 * t20105 - F::cast_from(0.85748036236139473944e-3_f64) * t93543 * t6273 + t100024 - F::cast_from(0.85748036236139473944e-3_f64) * t25580 * t19838 + F::cast_from(0.30488190661738479625e-2_f64) * t25577 * t6331 - F::cast_from(0.38110238327173099531e-3_f64) * t106923 - F::cast_from(0.63517063878621832551e-4_f64) * t93555 + F::cast_from(0.57165357490759649296e-3_f64) * t106926 - F::cast_from(0.30488190661738479625e-2_f64) * t100048 - t100051 + t100078;
    t106929
}
