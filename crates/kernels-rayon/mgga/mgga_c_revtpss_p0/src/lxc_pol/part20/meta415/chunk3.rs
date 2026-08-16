//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1538/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1538(t11273: f64, t12012: f64, t11160: f64, t11667: f64, t11675: f64, t11684: f64, t11824: f64, t15917: f64, t16081: f64, t3091: f64, t3092: f64, t3095: f64, t3097: f64, t3157: f64, t3241: f64, t42550: f64, t42610: f64, t43238: f64, t43242: f64, t43244: f64, t43254: f64, t43266: f64, t4786: f64) -> f64 {
    let t43268 = t11273 * t12012;
    let t43271 = -28.0_f64 / 243.0_f64 * t3241 * t11824 - 0.13550306960772657611e-2_f64 * t43238 - 0.3811023832717309953e-3_f64 * t43242 + 0.17149607247227894789e-2_f64 * t43244 * t3097 - 0.34299214494455789577e-2_f64 * t11675 * t11684 + 0.57165357490759649296e-3_f64 * t3091 * t3092 * t42610 * t3095 + 0.34299214494455789578e-2_f64 * t16081 * t3092 * t42550 * t43254 + 0.34299214494455789577e-2_f64 * t3091 * t3092 * t11160 * t4786 - 0.17149607247227894789e-2_f64 * t15917 * t11667 - 0.11433071498151929859e-2_f64 * t43266 - 0.27439371595564631662e-1_f64 * t43268 * t3157;
    t43271
}
