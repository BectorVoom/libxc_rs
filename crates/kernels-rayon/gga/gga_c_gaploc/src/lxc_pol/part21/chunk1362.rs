//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1362/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1362(t38313: f64, t493: f64, t105: f64, t1064: f64, t1079: f64, t1083: f64, t11988: f64, t12002: f64, t12008: f64, t1212: f64, t29871: f64, t29876: f64, t29879: f64, t31565: f64, t31568: f64, t3692: f64, t3696: f64, t3822: f64, t38299: f64, t3833: f64, t4141: f64, t419: f64, t492: f64) -> (f64, f64) {
    let t38314 = t493 * t38313;
    let t38320 = t29871 + t29876 - t29879 - 0.31616674039640166222e-2_f64 * t4141 * t12008 - t31565 - t31568 - 0.56910013271352299198e-1_f64 * t3822 * t1064 * t38299 + 0.56910013271352299198e-1_f64 * t3833 * t11988 - 0.28455006635676149599e-1_f64 * t1212 * t3696 - 0.12646669615856066488e-1_f64 * t1079 * t3696 - 0.7588001769513639893e-1_f64 * t1083 * t3696 - 0.56910013271352299198e-1_f64 * t419 * t12002 - 0.28455006635676149599e-1_f64 * t105 * t492 * t38314 + 0.12646669615856066488e-1_f64 * t1079 * t3692;
    (t38314, t38320)
}
