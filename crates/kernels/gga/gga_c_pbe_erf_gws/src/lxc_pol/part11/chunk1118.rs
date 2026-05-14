//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1118/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1118<F: Float>(t3886: F, t1109: F, t1115: F, t12130: F, t12232: F, t13112: F, t13187: F, t2409: F, t2416: F, t2501: F, t3055: F, t3207: F, t335: F, t338: F, t35000: F, t353: F, t35941: F, t3780: F, t3921: F, t43290: F, t44091: F, t44093: F, t44118: F, t44131: F, t44138: F, t829: F, t830: F, t9885: F, t9899: F) -> (F,) {
    let t50499 = t3886 * t3886;
    let t50514 = 35.0 / 12.0 * t35941 + t12130 * t829 * t830 * t2501 * t3780 / 8.0 - t3921 * t9885 / 8.0 + 7.0 / 12.0 * t44091 + 7.0 / 24.0 * t44093 - t3055 * t829 * t830 * t12232 * t1109 / 16.0 - t3921 * t9899 / 16.0 + t35000 * t13112 / 4.0 + t335 * t338 * t353 * t2416 * t50499 / 16.0 + 3.0 / 4.0 * t3207 * t2409 * t2501 * t13187 - 7.0 / 36.0 * t44118 - t1115 * t43290 / 12.0 + 7.0 / 12.0 * t44131 + 7.0 / 12.0 * t44138;
    (t50514,)
}
