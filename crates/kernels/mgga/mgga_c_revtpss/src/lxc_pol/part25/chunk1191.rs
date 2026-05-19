//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1191/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1191<F: Float>(t25970: F, t25974: F, t25976: F, t25980: F, t25984: F, t25989: F, t25990: F, t25992: F, t25994: F, t25998: F, t26033: F) -> F {
    let t26034 = -t25970 - t25974 + t25976 + t25980 + F::cast_from(0.85748036236139473944e-3_f64) * t25984 + t25989 - F::cast_from(0.17149607247227894789e-2_f64) * t25990 + F::cast_from(0.85748036236139473945e-2_f64) * t25992 - F::cast_from(0.42874018118069736972e-3_f64) * t25994 - F::cast_from(0.50820002809285328226e-4_f64) * t25998 + t26033;
    t26034
}
