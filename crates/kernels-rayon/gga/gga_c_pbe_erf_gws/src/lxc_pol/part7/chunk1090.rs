//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1090/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1090(t18709: f64, t18838: f64, t18850: f64, t18853: f64, t18863: f64, t18914: f64, t18916: f64, t18920: f64, t18924: f64, t18928: f64, t18933: f64, t18935: f64, t18939: f64, t19517: f64, t19521: f64, t19525: f64) -> f64 {
    let t19526 = t18709 + t18914 - t18838 + t18916 - t19517 + t18850 + t18920 + t18924 + t18853 - t19521 - t18863 + t19525 + t18928 - t18933 + t18935 + t18939;
    t19526
}
