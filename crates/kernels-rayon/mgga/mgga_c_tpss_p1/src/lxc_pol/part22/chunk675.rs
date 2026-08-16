//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 675/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk675(t1173: f64, t1186: f64, t1206: f64, t198: f64, t2281: f64, t2285: f64, t2310: f64, t3180: f64, t3182: f64, t3183: f64, t3184: f64, t3189: f64, t3192: f64, t3194: f64, t3196: f64, t3199: f64, t3201: f64, t3202: f64, t3205: f64, t509: f64) -> (f64, f64) {
    let t3209 = 8.0_f64 * t1173 * t1186;
    let t3210 = -t198 * t3202 * t3205 * t509 + 6.0_f64 * t1206 * t3183 * t3184 - t2281 - t2285 + t2310 - t3180 - t3182 + t3189 - t3192 + t3194 - t3196 + t3199 - t3201 - t3209;
    (t3209, t3210)
}
