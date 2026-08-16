//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1033/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1033(t1399: f64, t4782: f64, t1218: f64, t4793: f64, t18624: f64, t18626: f64, t18629: f64, t18631: f64, t18634: f64, t18636: f64, t18645: f64, t18647: f64, t18655: f64, t18658: f64) -> (f64, f64, f64) {
    let t18659 = t1399 * t4782;
    let t18660 = 0.41015588084031179722e4_f64 * t18659;
    let t18661 = t4793 * t1218;
    let t18662 = 0.70178680769462448852e1_f64 * t18661;
    let t18663 = -t18624 + t18626 - t18629 - t18631 - t18634 - t18636 - t18645 - t18647 + t18655 + t18658 - t18660 + t18662;
    (t18660, t18662, t18663)
}
