//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1042/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1042(t39: f64, t55: f64, t59: f64, t87: f64, t18667: f64, t18669: f64, t18701: f64, t18703: f64, t18705: f64, t18707: f64, t18709: f64, t18838: f64, t18850: f64, t18914: f64, t18916: f64, t18920: f64) -> (f64, f64) {
    let t18924 = 24.0_f64 * t39 * t55 * t59 * t87;
    let t18925 = -t18667 - t18669 + t18701 - t18703 + t18705 + t18707 + t18709 + t18914 - t18838 + t18916 + t18850 + t18920 + t18924;
    (t18924, t18925)
}
