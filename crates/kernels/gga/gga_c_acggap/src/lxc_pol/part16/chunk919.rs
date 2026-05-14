//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 919/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk919<F: Float>(t36133: F, t7799: F, t8506: F, t2290: F, t7780: F, t1423: F, t7746: F, t1507: F, t2020: F, t30120: F, t8793: F, t8948: F, t7839: F, t8787: F, t30689: F, t5286: F) -> (F, F, F, F, F, F, F, F, F) {
    let t36134 = 0.42874018118069736972e-3 * t36133;
    let t36135 = t7799 * t8506;
    let t36137 = t7780 * t2290;
    let t36139 = t7746 * t1423;
    let t36151 = t2020 * t1507;
    let t36152 = 7.0 / 144.0 * t36151;
    let t36156 = t30120 * t8793;
    let t36157 = 0.62896184579208304136e-3 * t36156;
    let t36162 = t30120 * t8948;
    let t36163 = 0.42874018118069736972e-3 * t36162;
    let t36175 = t7839 * t8787;
    let t36176 = 0.94344276868812456204e-3 * t36175;
    let t36177 = t30689 * t5286;
    (t36134, t36135, t36137, t36139, t36152, t36157, t36163, t36176, t36177)
}
