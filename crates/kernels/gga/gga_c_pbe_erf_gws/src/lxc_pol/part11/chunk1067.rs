//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1067/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1067<F: Float>(t1109: F, t1115: F, t11375: F, t1144: F, t1161: F, t13119: F, t13121: F, t2409: F, t3066: F, t34922: F, t353: F, t3721: F, t3722: F, t3886: F, t3887: F, t3917: F, t43451: F, t43643: F, t43734: F, t44025: F, t44149: F, t46707: F, t46862: F, t47050: F, t859: F, t8629: F, t8793: F, t9296: F, t9885: F) -> (F,) {
    let t49147 = -t8629 * t859 * t353 * t3722 * t1109 / 8.0 + t34922 * t13121 / 12.0 + 3.0 / 4.0 * t8793 * t44025 + t11375 * t859 * t353 * t43451 * t1109 / 16.0 + t8629 * t859 * t1144 * t13119 / 8.0 + t8793 * t43643 / 4.0 + 7.0 / 24.0 * t43734 - t3917 * t9885 / 8.0 - 3.0 / 8.0 * t3066 * t2409 * t9296 * t3721 * t3886 + 3.0 / 4.0 * t1115 * t46707 + t8629 * t859 * t353 * t47050 * t1161 / 24.0 + t8629 * t859 * t353 * t3887 * t1109 / 16.0 - t8793 * t44149 / 2.0 - 3.0 / 4.0 * t8793 * t46862;
    (t49147,)
}
