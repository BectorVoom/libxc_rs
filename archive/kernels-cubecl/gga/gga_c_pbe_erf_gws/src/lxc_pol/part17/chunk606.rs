//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 606/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk606<F: Float>(t203: F, t2814: F, t184: F, t221: F, t1755: F, t1756: F, t2760: F, t2763: F, t2766: F, t2770: F, t173: F, t199: F) -> (F, F, F, F, F, F, F) {
    let t2815 = t203 * t2814;
    let t2816 = t2815 * t184;
    let t2818 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t2816 * t221;
    let t2824 = -t1755 - F::cast_from(0.62972222222222222223e-3_f64) * t1756 - F::cast_from(0.62972222222222222223e-3_f64) * t2760 + F::cast_from(0.12594444444444444445e-2_f64) * t2763 - F::cast_from(0.37783333333333333334e-2_f64) * t2766 + F::cast_from(0.37783333333333333334e-2_f64) * t2770;
    let t2825 = t173 * t2824;
    let t2826 = t2825 * t184;
    let t2828 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t2826 * t199;
    (t2815, t2816, t2818, t2824, t2825, t2826, t2828)
}
