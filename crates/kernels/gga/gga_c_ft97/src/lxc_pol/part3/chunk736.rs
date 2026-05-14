//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 736/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk736<F: Float>(t12969: F, t3478: F, t12968: F, t1017: F, t2178: F, t3483: F, t13140: F, t13153: F, t3425: F, t11593: F, t12676: F, t16979: F, t16983: F, t16986: F, t16990: F, t16993: F, t16998: F, t17003: F, t17008: F, t17013: F, t17018: F, t17023: F, t1901: F, t446: F) -> (F,) {
    let t17026 = t12969 * t3478;
    let t17027 = t12968 * t17026;
    let t17030 = t2178 * t1017;
    let t17031 = t17030 * t3483;
    let t17032 = t13140 * t17031;
    let t17035 = t13153 * t3425;
    let t17038 = 2.0 / 3.0 * t446 * t16979 - t446 * t16983 / 9.0 + t12676 - 2.0 / 27.0 * t16986 + 4.0 / 9.0 * t11593 * t16990 + 2.0 / 9.0 * t1901 * t16993 + 2.0 / 9.0 * t1901 * t16998 + 4.0 / 9.0 * t11593 * t17003 + t1901 * t17008 / 9.0 + t1901 * t17013 / 9.0 - 2.0 / 9.0 * t1901 * t17018 - 2.0 / 9.0 * t1901 * t17023 - 4.0 / 3.0 * t1901 * t17027 - 4.0 / 3.0 * t1901 * t17032 + 2.0 / 9.0 * t1901 * t17035;
    (t17038,)
}
