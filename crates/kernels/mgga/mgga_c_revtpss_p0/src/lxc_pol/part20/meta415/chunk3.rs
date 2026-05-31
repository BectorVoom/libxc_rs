//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1538/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1538<F: Float>(t11273: F, t12012: F, t11160: F, t11667: F, t11675: F, t11684: F, t11824: F, t15917: F, t16081: F, t3091: F, t3092: F, t3095: F, t3097: F, t3157: F, t3241: F, t42550: F, t42610: F, t43238: F, t43242: F, t43244: F, t43254: F, t43266: F, t4786: F) -> F {
    let t43268 = t11273 * t12012;
    let t43271 = -F::cast_from(28.0_f64) / F::cast_from(243.0_f64) * t3241 * t11824 - F::cast_from(0.13550306960772657611e-2_f64) * t43238 - F::cast_from(0.3811023832717309953e-3_f64) * t43242 + F::cast_from(0.17149607247227894789e-2_f64) * t43244 * t3097 - F::cast_from(0.34299214494455789577e-2_f64) * t11675 * t11684 + F::cast_from(0.57165357490759649296e-3_f64) * t3091 * t3092 * t42610 * t3095 + F::cast_from(0.34299214494455789578e-2_f64) * t16081 * t3092 * t42550 * t43254 + F::cast_from(0.34299214494455789577e-2_f64) * t3091 * t3092 * t11160 * t4786 - F::cast_from(0.17149607247227894789e-2_f64) * t15917 * t11667 - F::cast_from(0.11433071498151929859e-2_f64) * t43266 - F::cast_from(0.27439371595564631662e-1_f64) * t43268 * t3157;
    t43271
}
