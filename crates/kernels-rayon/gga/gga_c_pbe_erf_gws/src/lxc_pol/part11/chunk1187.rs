//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1187/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1187(t48140: f64, t48142: f64, t48148: f64, t48150: f64, t48152: f64, t48153: f64, t48155: f64, t48158: f64, t48159: f64, t48160: f64, t48162: f64, t48165: f64, t48169: f64, t48173: f64, t48175: f64, t48179: f64, t48183: f64, t48187: f64, t48191: f64, t48195: f64, t48198: f64, t48201: f64) -> (f64, f64) {
    let t48678 = -t48140 + t48142 - t48148 + t48150 + t48152 - t48153 - t48155 - t48158 - t48159 - t48160 + t48162;
    let t48679 = t48165 + t48169 + t48173 - t48175 - t48179 + t48183 + t48187 - t48191 - t48195 - t48198 - t48201;
    (t48678, t48679)
}
