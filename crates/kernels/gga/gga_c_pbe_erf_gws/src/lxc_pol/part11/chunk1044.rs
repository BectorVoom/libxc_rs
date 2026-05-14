//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1044/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1044<F: Float>(t48165: F, t48169: F, t48173: F, t48175: F, t48179: F, t48183: F, t48187: F, t48191: F, t48195: F, t48198: F, t48201: F, t48203: F, t48207: F, t48213: F, t48215: F, t48219: F, t48223: F, t48225: F, t48227: F, t48229: F, t48231: F, t48232: F) -> (F, F) {
    let t48679 = t48165 + t48169 + t48173 - t48175 - t48179 + t48183 + t48187 - t48191 - t48195 - t48198 - t48201;
    let t48681 = t48203 - t48207 - t48213 + t48215 + t48219 + t48223 - t48225 + t48227 + t48229 - t48231 + t48232;
    (t48679, t48681)
}
