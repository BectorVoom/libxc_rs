//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2045/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2045<F: Float>(t1426: F, t97960: F, t7063: F, t7286: F, t27852: F, t689: F, t25904: F, t25899: F, t25950: F, t27888: F, t25953: F, t27884: F) -> (F, F, F, F, F, F) {
    let t97961 = t97960 * t1426;
    let t97962 = t7063 * t97961;
    let t97964 = F::cast_from(0.25702851531048074406e-1_f64) * t97962 * t7286;
    let t97966 = t27852 * t689;
    let t97968 = F::cast_from(0.14456046980341999104e-1_f64) * t25904 * t97966;
    let t97974 = F::cast_from(0.25702851531048074406e-1_f64) * t25899 * t97966;
    let t97976 = F::cast_from(0.25702851531048074406e-1_f64) * t25950 * t27888;
    let t97985 = t27884 * t25953;
    (t97961, t97964, t97968, t97974, t97976, t97985)
}
