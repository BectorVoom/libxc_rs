//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 641/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk641(t12000: f64, t493: f64, t492: f64, t11986: f64, t550: f64, t1365: f64, t10184: f64, t10187: f64, t10195: f64, t10198: f64, t10229: f64, t10236: f64, t10238: f64, t10240: f64, t10245: f64, t105: f64, t1358: f64, t3692: f64, t419: f64, t9207: f64, t9210: f64) -> (f64, f64, f64) {
    let t12001 = t493 * t12000;
    let t12002 = t492 * t12001;
    let t12007 = t550 * t11986;
    let t12008 = t1365 * t12007;
    let t12011 = t10184 + t10187 - 0.28455006635676149599e-1_f64 * t105 * t12002 + 0.28455006635676149599e-1_f64 * t419 * t3692 + 0.31616674039640166221e-2_f64 * t1358 * t12008 - t10195 - t10198 - t9207 + t9210 + t10229 + t10236 - t10238 - t10240 + t10245;
    (t12001, t12007, t12011)
}
