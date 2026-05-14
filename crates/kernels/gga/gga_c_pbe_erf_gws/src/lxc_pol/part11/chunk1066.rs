//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1066/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1066<F: Float>(t3802: F, t20133: F, t3703: F, t3717: F, t2083: F, t3776: F, t1109: F, t1115: F, t1118: F, t11375: F, t1162: F, t12182: F, t13121: F, t13290: F, t20138: F, t2409: F, t326: F, t35000: F, t35057: F, t353: F, t3737: F, t3912: F, t43323: F, t43549: F, t43671: F, t4386: F, t46763: F, t46867: F, t825: F, t831: F, t833: F, t859: F, t8599: F, t8629: F, t8793: F, t9241: F) -> (F, F, F, F, F) {
    let t49063 = t3802 * t3802;
    let t49064 = t20133 * t49063;
    let t49087 = t3703 * t3717;
    let t49092 = t2083 * t3776;
    let t49102 = -t1115 * t43323 + t1115 * t46763 / 24.0 + t326 * t49064 * t20138 * t833 / 96.0 - 7.0 / 36.0 * t43549 - t11375 * t4386 * t353 * t1118 * t13290 / 6.0 - 3.0 / 8.0 * t8629 * t8599 * t353 * t3737 * t1109 + t8793 * t46867 / 4.0 + t35057 * t12182 / 4.0 + t35000 * t13121 / 8.0 - 7.0 / 6.0 * t43671 - 3.0 / 2.0 * t9241 * t2409 * t831 * t49087 + t3912 * t49092 * t825 * t833 / 32.0 - t11375 * t859 * t353 * t1162 * t13290 / 12.0;
    (t49063, t49064, t49087, t49092, t49102)
}
