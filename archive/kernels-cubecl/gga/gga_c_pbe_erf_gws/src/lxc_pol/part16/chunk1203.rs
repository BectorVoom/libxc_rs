//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1203/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1203<F: Float>(t1176: F, t2332: F, t903: F, t3993: F, t13788: F, t13972: F, t13865: F, t51666: F, t14001: F, t2412: F, t1180: F, t6589: F) -> (F, F, F, F, F, F) {
    let t51818 = t1176 * t2332 * t903;
    let t51819 = t51818 * t3993;
    let t51827 = t13972 * t13788;
    let t51829 = t51666 * t13865;
    let t51864 = t14001 * t2412;
    let t51869 = t1176 * t6589 * t1180;
    (t51818, t51819, t51827, t51829, t51864, t51869)
}
