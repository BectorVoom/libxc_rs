//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1168/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1168<F: Float>(t113168: F, t1486: F, t28507: F, t681: F, t1900: F, t6: F, t845: F, t91: F, t43917: F, t6318: F, t28731: F, t99312: F, t28726: F, t3051: F, t6307: F, t192: F, t2781: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t113169 = 2.0 / 3.0 * t113168;
    let t113176 = t1486 * t681 * t28507;
    let t113177 = 2.0 / 3.0 * t113176;
    let t113190 = t91 * t845 * t6 * t1900;
    let t113191 = t43917 * t6318;
    let t113195 = t99312 * t28731;
    let t113196 = t113195 / 18.0;
    let t113201 = t99312 * t28726;
    let t113202 = t113201 / 18.0;
    let t113208 = t6307 * t3051;
    let t113214 = t192 * t2781;
    (t113169, t113176, t113177, t113190, t113191, t113195, t113196, t113201, t113202, t113208, t113214)
}
