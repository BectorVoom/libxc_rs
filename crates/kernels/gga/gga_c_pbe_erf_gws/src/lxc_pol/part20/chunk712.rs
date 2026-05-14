//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 712/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk712<F: Float>(t1322: F, t4607: F, t2704: F, t2718: F, t4518: F, t4521: F, t4524: F, t4529: F, t4531: F, t4533: F, t456: F, t1314: F, t455: F, t1231: F, t440: F, t441: F) -> (F, F, F, F, F, F, F) {
    let t4608 = t4607 * t1322;
    let t4619 = -0.34523333333333333333e1 * t4518 + 0.23015555555555555556e1 * t4521 - 0.26851481481481481482e1 * t4524 - 0.93932222222222222223e0 * t2704 + 0.73355e-1 * t4529 - 0.14671e0 * t4531 - 0.17116166666666666667e0 * t4533 - 0.36793333333333333333e0 * t2718;
    let t4620 = t4619 * t456;
    let t4623 = t1314 * t1322;
    let t4624 = t4623 * t455;
    let t4630 = t1231 * t440;
    let t4631 = t4630 * t441;
    (t4608, t4619, t4620, t4623, t4624, t4630, t4631)
}
