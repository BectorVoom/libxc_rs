//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 858/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk858(t35760: f64, t666: f64, t461: f64, t6903: f64, t231: f64, t6837: f64, t1403: f64, t1526: f64, t2: f64, t2320: f64, t33545: f64, t33557: f64, t342: f64, t343: f64, t35757: f64, t6895: f64, t6900: f64, t7426: f64, t7427: f64) -> (f64, f64, f64, f64) {
    let t35761 = t666 * t35760;
    let t35766 = t461 * t6903;
    let t35772 = t231 * t6837;
    let t35777 = (-t35757 * t7427 / 6.0_f64 + t33545 + t1403 * t35761 / 18.0_f64 + t1403 * t6900 / 3.0_f64 - t7426 * t35766 / 6.0_f64 - t33557 - t1526 * t2320 * t6895 / 12.0_f64 - t342 * t343 * t35772 / 4.0_f64) * t2;
    (t35761, t35766, t35772, t35777)
}
