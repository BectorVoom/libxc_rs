//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 698/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk698(t29128: f64, t29130: f64, t15195: f64, t6361: f64, t4162: f64, t6360: f64, t15369: f64, t25271: f64, t4167: f64, t15460: f64, t1901: f64, t24903: f64, t29098: f64, t29101: f64, t29104: f64, t29107: f64, t29111: f64, t29113: f64, t29116: f64, t29120: f64, t29124: f64, t446: f64) -> (f64, f64, f64) {
    let t29131 = t29128 * t29130;
    let t29134 = t15195 * t6361;
    let t29137 = t6360 * t4162;
    let t29138 = t15369 * t29137;
    let t29141 = t25271 * t4167;
    let t29142 = t15460 * t29141;
    let t29145 = t1901 * t29098 / 9.0_f64 - t446 * t29101 / 3.0_f64 - t446 * t29104 / 3.0_f64 + t1901 * t29107 / 9.0_f64 - t24903 / 27.0_f64 - t29111 / 27.0_f64 + t1901 * t29113 / 9.0_f64 - t446 * t29116 / 3.0_f64 - t446 * t29120 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t1901 * t29124 - 2.0_f64 * t1901 * t29131 + t1901 * t29134 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t1901 * t29138 - 2.0_f64 / 3.0_f64 * t1901 * t29142;
    (t29137, t29141, t29145)
}
