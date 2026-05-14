//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 821/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk821<F: Float>(t148: F, t18411: F, t1464: F, t700: F, t242: F, t5984: F, t1286: F, t174: F, t4708: F, t155: F, t4658: F, t4662: F, t4508: F, t4511: F, t1268: F, t4537: F) -> (F, F, F, F, F, F, F) {
    let t18413 = 0.83762820535504401876e-1 * t148 * t18411;
    let t18415 = 0.2010307692852105645e1 * t1464 * t700;
    let t18419 = 0.2010307692852105645e1 * t5984 * t242;
    let t18424 = 0.14246666666666666667e0 * t174 * t4708 * t1286;
    let t18428 = 0.36845452142031360636e2 * t174 * t155 * t4658 * t4662;
    let t18432 = 0.68733717152873822009e1 * t174 * t155 * t4508 * t4511;
    let t18435 = 0.71233333333333333333e-1 * t174 * t1268 * t4537;
    (t18413, t18415, t18419, t18424, t18428, t18432, t18435)
}
