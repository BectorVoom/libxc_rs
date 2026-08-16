//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 912/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk912<F: Float>(t4560: F, t461: F, t409: F, t4743: F, t31: F, t4: F, t4573: F, t1318: F, t1216: F, t1321: F, t470: F, t1289: F) -> (F, F, F, F, F, F, F, F) {
    let t18623 = t4560 * t461;
    let t18624 = F::cast_from(576.0_f64) * t18623;
    let t18625 = t409 * t4743;
    let t18626 = F::cast_from(16.0_f64) * t18625;
    let t18629 = F::cast_from(0.11483710345679012345e-1_f64) * t4 * t4573 * t31;
    let t18637 = t1318 * t1318;
    let t18638 = F::cast_from(1.0_f64) / t18637;
    let t18639 = t1216 * t1216;
    let t18641 = t1321 * t1321;
    let t18642 = F::cast_from(1.0_f64) / t18641;
    let t18645 = F::cast_from(0.91080982599109921211e5_f64) * t470 * t18638 * t18639 * t18642;
    let t18648 = t1289 * t1289;
    (t18624, t18626, t18629, t18638, t18639, t18642, t18645, t18648)
}
