//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1222/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1222<F: Float>(t1211: F, t21885: F, t804: F, t20091: F, t4090: F, t2416: F, t4110: F, t22509: F, t4099: F, t4083: F, t4424: F, t51869: F) -> (F, F, F, F, F, F, F) {
    let t52105 = t1211 * t21885;
    let t52112 = t804 * t1211;
    let t52159 = t20091 * t4090;
    let t52191 = t2416 * t4110;
    let t52251 = t22509 * t4099;
    let t52353 = t4424 * t4083;
    let t52525 = F::cast_from(595.0_f64) / F::cast_from(5184.0_f64) * t51869;
    (t52105, t52112, t52159, t52191, t52251, t52353, t52525)
}
