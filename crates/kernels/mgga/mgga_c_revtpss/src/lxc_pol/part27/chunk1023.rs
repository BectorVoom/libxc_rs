//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1023/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1023<F: Float>(t26028: F, t3940: F, t3926: F, t7264: F, t26003: F, t26006: F, t26007: F, t26011: F, t26013: F, t26016: F, t26018: F, t26022: F, t26025: F, t25970: F, t25974: F, t25976: F, t25980: F, t25984: F, t25989: F, t25990: F, t25992: F, t25994: F, t25998: F) -> (F,) {
    let t26029 = t26028 * t3940;
    let t26031 = t7264 * t3926;
    let t26033 = t26003 + t26006 - t26007 / 48.0 - t26011 + t26013 + t26016 + t26018 / 16.0 + t26022 + 0.40015750243531754508e-2 * t26025 + 0.34299214494455789578e-2 * t26029 - 0.42874018118069736972e-3 * t26031;
    let t26034 = -t25970 - t25974 + t25976 + t25980 + 0.85748036236139473944e-3 * t25984 + t25989 - 0.17149607247227894789e-2 * t25990 + 0.85748036236139473945e-2 * t25992 - 0.42874018118069736972e-3 * t25994 - 0.50820002809285328226e-4 * t25998 + t26033;
    (t26034,)
}
