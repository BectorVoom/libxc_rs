//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1295/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1295<F: Float>(t33: F, t3841: F, t6416: F, t1113: F, t20256: F, t21918: F, t2255: F, t516: F, t5557: F, t162: F, t21917: F, t187: F, t1450: F, t6922: F, t6785: F, t9605: F, t3874: F, t5824: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t21923 = t3841 * t6416;
    let t21929 = piecewise3(t34, 0.0, -8.0 / 27.0 * t21918 * t1113 - 16.0 / 9.0 * t5557 * t2255 + 4.0 / 9.0 * t21923 * t1113 + 4.0 / 3.0 * t516 * t20256);
    let t21931 = (t21917 + t21929) * t162;
    let t21933 = 0.19751673498613801407e-1 * t21931 * t187;
    let t21937 = t6922 * t1450;
    let t21944 = t9605 * t6785;
    let t21949 = t3874 * t5824;
    (t21931, t21933, t21937, t21944, t21949)
}
