//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 827/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk827<F: Float>(t2561: F, t7669: F, t587: F, t197: F, t2620: F, t1660: F, t331: F, t1802: F, t1885: F, t2566: F, t5129: F, t597: F) -> (F, F, F, F, F, F) {
    let t7670 = t7669 * t2561;
    let t7672 = F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t587 * t7670;
    let t7694 = t2620 * t197;
    let t7698 = t331 * t1660;
    let t7699 = t7698 * t197;
    let t7703 = t1885 * t1802;
    let t7713 = t5129 * t2566;
    let t7715 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t587 * t7713;
    let t7720 = t2620 * t597;
    (t7672, t7694, t7699, t7703, t7715, t7720)
}
