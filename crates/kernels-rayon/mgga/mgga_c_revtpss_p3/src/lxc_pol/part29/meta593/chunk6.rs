//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1983/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1983(t98281: f64, t98285: f64, t94542: f64, t94546: f64, t94548: f64, t94552: f64, t94554: f64, t94557: f64, t94559: f64, t94561: f64, t94565: f64, t96358: f64, t96359: f64) -> f64 {
    let t102567 = 0.22866142996303859718e-3_f64 * t98281;
    let t102569 = 0.72286371995927450867e-4_f64 * t98285;
    let t102570 = -0.2032800112371413129e-3_f64 * t94542 - 0.18140473443734395377e0_f64 * t94546 + 0.16006300097412701803e-1_f64 * t94548 - 0.57165357490759649296e-4_f64 * t94552 - 0.6097638132347695925e-3_f64 * t94554 + 0.28582678745379824648e-4_f64 * t94557 - 0.80031500487063509015e-1_f64 * t94559 + 0.10164000561857065645e-2_f64 * t94561 + t102567 - 0.36143185997963725434e-4_f64 * t94565 - t96358 - t96359 - t102569;
    t102570
}
