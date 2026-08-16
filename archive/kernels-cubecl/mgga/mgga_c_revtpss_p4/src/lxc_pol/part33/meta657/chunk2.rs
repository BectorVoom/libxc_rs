//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2114/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2114<F: Float>(t29658: F, t686: F, t72: F, t7058: F, t7064: F, t105934: F, t105937: F, t105939: F, t105947: F, t105949: F, t27349: F, t92858: F, t93349: F, t98803: F, t98806: F, t98811: F, t98814: F, t98817: F, t99414: F) -> F {
    let t105953 = t29658 * t72 * t686;
    let t105954 = t7058 * t105953;
    let t105956 = t7064 * t105953;
    let t105958 = -t98803 + t98806 + F::cast_from(0.14456046980341999104e-1_f64) * t105934 + t98811 - t98814 - t98817 - F::cast_from(0.51405703062096148813e-1_f64) * t105937 - F::cast_from(0.25702851531048074406e-1_f64) * t105939 + F::cast_from(0.52041769129231196772e1_f64) * t93349 * t99414 * t27349 + F::cast_from(0.72280234901709995518e-2_f64) * t105947 + F::cast_from(0.28912093960683998207e-1_f64) * t105949 - F::cast_from(0.73171657588172351096e-2_f64) * t92858 + F::cast_from(0.72280234901709995518e-2_f64) * t105954 - F::cast_from(0.12851425765524037203e-1_f64) * t105956;
    t105958
}
