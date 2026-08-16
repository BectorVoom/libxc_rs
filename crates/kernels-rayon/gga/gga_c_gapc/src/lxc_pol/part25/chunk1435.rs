//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1435/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1435(t33969: f64, t36773: f64, t36774: f64, t36775: f64, t36777: f64, t36778: f64, t36779: f64, t36780: f64, t36781: f64, t36782: f64, t36783: f64, t34036: f64, t36800: f64, t36801: f64, t36802: f64, t36803: f64, t36804: f64, t36805: f64, t36806: f64, t36807: f64, t36808: f64, t36809: f64) -> (f64, f64) {
    let t38779 = -t36773 - t36774 + t36775 - 0.25301106770833333334e-5_f64 * t33969 + t36777 - t36778 + t36779 + t36780 - t36781 - t36782 - t36783;
    let t38788 = -0.11666621455439814815e-3_f64 * t34036 + t36800 - t36801 - t36802 + t36803 - t36804 + t36805 + t36806 + t36807 + t36808 + t36809;
    (t38779, t38788)
}
