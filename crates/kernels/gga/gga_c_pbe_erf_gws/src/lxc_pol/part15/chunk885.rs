//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 885/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk885<F: Float>(t587: F, t7666: F, t197: F, t5283: F, t2561: F, t1000: F, t1866: F, t1827: F, t1821: F, t7350: F, t2559: F, t7326: F) -> (F, F, F, F, F) {
    let t7668 = F::cast_from(32.0_f64) / F::cast_from(135.0_f64) * t587 * t7666;
    let t7669 = t5283 * t197;
    let t7670 = t7669 * t2561;
    let t7672 = F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t587 * t7670;
    let t7673 = t1000 * t1866;
    let t7674 = t1827 * t7673;
    let t7676 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t587 * t7674;
    let t7677 = t1821 * t7350;
    let t7679 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t587 * t7677;
    let t7680 = t2559 * t7326;
    (t7668, t7672, t7676, t7679, t7680)
}
