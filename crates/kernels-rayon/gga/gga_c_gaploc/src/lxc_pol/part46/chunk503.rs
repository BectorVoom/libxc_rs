//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 503/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk503(t550: f64, t9198: f64, t1365: f64, t1063: f64, t1358: f64, t2268: f64, t3138: f64, t419: f64, t9162: f64, t9165: f64, t9168: f64, t9173: f64, t9178: f64, t9183: f64, t9186: f64, t9190: f64, t9195: f64) -> (f64, f64) {
    let t9199 = t550 * t9198;
    let t9200 = t1365 * t9199;
    let t9203 = -0.28455006635676149599e-1_f64 * t419 * t3138 - 0.28455006635676149599e-1_f64 * t1063 * t9162 - 0.28455006635676149599e-1_f64 * t1063 * t9165 + 0.56910013271352299198e-1_f64 * t2268 * t9168 + 0.56910013271352299198e-1_f64 * t2268 * t9173 - 0.17073003981405689759e0_f64 * t2268 * t9178 + 0.34146007962811379518e0_f64 * t2268 * t9183 - 0.19918504644973304719e0_f64 * t2268 * t9186 - 0.85365019907028448797e-1_f64 * t2268 * t9190 - 0.63233348079280332442e-2_f64 * t1358 * t9195 + 0.31616674039640166221e-2_f64 * t1358 * t9200;
    (t9199, t9203)
}
