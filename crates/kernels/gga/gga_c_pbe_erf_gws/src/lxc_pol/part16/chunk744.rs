//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 744/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk744<F: Float>(t1336: F, t428: F, t726: F, t728: F, t4607: F, t4734: F, t4737: F, t470: F, t1396: F, t1399: F, t449: F, t456: F, t4619: F) -> (F, F, F, F, F, F) {
    let t4755 = t1336 * t428;
    let t4757 = F::new(1.0) / t726;
    let t4767 = F::new(1.0) / t728;
    let t4782 = t4734 * t4607 * t4737;
    let t4783 = t470 * t4782;
    let t4784 = F::new(0.1025389702100779493e4) * t4783;
    let t4785 = t1399 * t1396;
    let t4788 = t449 * t4619 * t456;
    (t4755, t4757, t4767, t4784, t4785, t4788)
}
