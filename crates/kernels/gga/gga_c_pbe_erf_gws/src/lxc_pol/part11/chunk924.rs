//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 924/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk924<F: Float>(t13173: F, t2366: F, t833: F, t13207: F, t4414: F, t11347: F, t3916: F, t1161: F, t353: F, t35553: F, t4386: F, t12182: F, t26958: F, t36114: F, t3733: F, t9848: F) -> (F, F, F, F, F, F, F) {
    let t43466 = t13173 * t2366 * t833;
    let t43487 = t4414 * t13207;
    let t43549 = t3916 * t11347 * t833;
    let t43643 = t4386 * t353 * t35553 * t1161;
    let t43671 = t26958 * t12182;
    let t43734 = t36114 * t3733;
    let t43740 = t3916 * t9848;
    (t43466, t43487, t43549, t43643, t43671, t43734, t43740)
}
