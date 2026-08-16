//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1047/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1047(t40890: f64, t40896: f64, t43243: f64, t43254: f64, t43257: f64, t43260: f64, t43263: f64, t43265: f64, t43267: f64, t43269: f64, t43274: f64, t43282: f64, t43286: f64, t47723: f64, t47731: f64, t47734: f64, t47737: f64, t47740: f64) -> f64 {
    let t51047 = 0.30762104920568897134e-1_f64 * t47723 - t43243 + t43254 + t43257 + 0.64087718584518535698e-3_f64 * t47731 + t43260 + t43263 + t43265 - t43267 - t43269 - 0.46143157380853345702e-1_f64 * t47734 - 0.46143157380853345702e-1_f64 * t47737 - 0.46143157380853345702e-1_f64 * t47740 - t43274 + 0.25635087433807414279e-2_f64 * t40890 - t43282 - 0.17090058289204942852e-2_f64 * t40896 - t43286;
    t51047
}
