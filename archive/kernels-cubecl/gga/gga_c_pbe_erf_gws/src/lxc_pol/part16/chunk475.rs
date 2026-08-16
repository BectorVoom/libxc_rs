//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 475/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk475<F: Float>(t1986: F, t226: F, t1354: F, t225: F, t666: F, t679: F, t1626: F, t1629: F, t1633: F, t1637: F, t1647: F, t1650: F, t1654: F, t1658: F, t231: F) -> (F, F, F, F) {
    let t1988 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t226 * t1986;
    let t1989 = t1354 * t225;
    let t1992 = t666 * t679;
    let t1994 = t1988 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1989 * t231 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1992 - t1626 + t1629 + t1633 + t1637 + t1647 + t1650 + t1654 + t1658;
    (t1988, t1989, t1992, t1994)
}
