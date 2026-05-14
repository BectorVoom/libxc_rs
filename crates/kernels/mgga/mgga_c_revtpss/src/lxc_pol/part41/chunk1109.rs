//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1109/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1109<F: Float>(t10846: F, t10885: F, t10888: F, t10891: F, t10900: F, t18491: F, t18495: F, t18500: F, t18507: F, t18511: F, t18515: F, t18518: F, t18521: F, t2730: F, t4362: F, t851: F) -> (F,) {
    let t18524 = 0.10003937560882938627e-2 * t18491 - 0.25724410870841842183e-1 * t851 * t18495 + 0.85748036236139473944e-2 * t851 * t18500 + 0.13552000749142754193e-3 * t10846 - t10885 + 0.10164000561857065645e-4 * t10888 - 35.0 / 216.0 * t10891 + t2730 * t18507 / 16.0 - t10900 * t18511 / 4.0 + t2730 * t18515 / 8.0 - 0.80031500487063509015e-2 * t18518 + 0.42874018118069736972e-3 * t4362 * t18521;
    (t18524,)
}
