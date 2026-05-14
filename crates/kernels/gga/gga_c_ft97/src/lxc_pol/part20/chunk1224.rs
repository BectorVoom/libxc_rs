//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1224/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1224<F: Float>(t1486: F, t28502: F, t681: F, t4129: F, t6260: F, t193: F, t2781: F, t28507: F, t25027: F, t2756: F, t7021: F, t852: F, t28719: F, t6308: F, t856: F, t1900: F, t6: F, t845: F, t91: F) -> (F, F, F, F, F, F, F, F, F) {
    let t113168 = t1486 * t681 * t28502;
    let t113169 = 2.0 / 3.0 * t113168;
    let t113170 = t6260 * t4129;
    let t113173 = t1486 * t193 * t2781 * t113170;
    let t113176 = t1486 * t681 * t28507;
    let t113177 = 2.0 / 3.0 * t113176;
    let t113181 = t25027 * t193 * t852 * t7021 * t2756;
    let t113186 = t6308 * t193 * t852 * t28719 * t856;
    let t113190 = t91 * t845 * t6 * t1900;
    (t113168, t113169, t113170, t113173, t113176, t113177, t113181, t113186, t113190)
}
