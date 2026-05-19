//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 758/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk758<F: Float>(t4656: F, t4741: F, t60: F, t40: F, t1322: F, t4605: F, t4607: F, t470: F, t1336: F, t461: F, t428: F, t726: F) -> (F, F, F, F, F) {
    let t4742 = t4656 + t4741;
    let t4743 = t60 * t4742;
    let t4744 = t40 * t4743;
    let t4749 = t4605 * t4607 * t1322;
    let t4750 = t470 * t4749;
    let t4751 = F::cast_from(0.1038945353962551798e3_f64) * t4750;
    let t4753 = t1336 * t461;
    let t4755 = t1336 * t428;
    let t4757 = F::new(1.0) / t726;
    (t4744, t4751, t4753, t4755, t4757)
}
