//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 987/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk987<F: Float>(t3133: F, t6183: F, t2134: F, t1133: F, t874: F, t2171: F, t4386: F, t2168: F, t6185: F, t3179: F, t6331: F, t2146: F) -> (F, F, F, F, F, F) {
    let t8824 = t6183 * t3133;
    let t8826 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t2134 * t8824;
    let t8827 = t1133 * t874;
    let t8828 = t8827 * t2171;
    let t8829 = t4386 * t8828;
    let t8831 = t2168 * t8829 / F::cast_from(24.0_f64);
    let t8832 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t6185;
    let t8833 = t6331 * t3179;
    let t8835 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t2146 * t8833;
    (t8826, t8827, t8828, t8831, t8832, t8835)
}
