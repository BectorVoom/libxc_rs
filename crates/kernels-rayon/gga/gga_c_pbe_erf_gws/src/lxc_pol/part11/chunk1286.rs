//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1286/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1286(t50110: f64, t50115: f64, t50116: f64, t50128: f64, t50135: f64, t50137: f64, t50146: f64, t50158: f64, t50160: f64, t50162: f64, t50168: f64, t50187: f64, t50189: f64, t50193: f64, t50201: f64, t50206: f64, t50207: f64, t50212: f64, t50219: f64, t50220: f64, t50230: f64, t50231: f64) -> (f64, f64) {
    let t50586 = t50110 - t50115 + t50116 - t50128 + t50135 + t50137 - t50146 + t50158 - t50160 - t50162 - t50168;
    let t50587 = -t50187 + t50189 + t50193 - t50201 - t50206 - t50207 - t50212 - t50219 - t50220 - t50230 + t50231;
    (t50586, t50587)
}
