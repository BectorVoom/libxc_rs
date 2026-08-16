//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1364/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1364<F: Float>(t38313: F, t493: F, t105: F, t1064: F, t1079: F, t1083: F, t11988: F, t12002: F, t12008: F, t1212: F, t29871: F, t29876: F, t29879: F, t31565: F, t31568: F, t3692: F, t3696: F, t3822: F, t38299: F, t3833: F, t4141: F, t419: F, t492: F) -> (F, F) {
    let t38314 = t493 * t38313;
    let t38320 = t29871 + t29876 - t29879 - F::cast_from(0.31616674039640166222e-2_f64) * t4141 * t12008 - t31565 - t31568 - F::cast_from(0.56910013271352299198e-1_f64) * t3822 * t1064 * t38299 + F::cast_from(0.56910013271352299198e-1_f64) * t3833 * t11988 - F::cast_from(0.28455006635676149599e-1_f64) * t1212 * t3696 - F::cast_from(0.12646669615856066488e-1_f64) * t1079 * t3696 - F::cast_from(0.7588001769513639893e-1_f64) * t1083 * t3696 - F::cast_from(0.56910013271352299198e-1_f64) * t419 * t12002 - F::cast_from(0.28455006635676149599e-1_f64) * t105 * t492 * t38314 + F::cast_from(0.12646669615856066488e-1_f64) * t1079 * t3692;
    (t38314, t38320)
}
