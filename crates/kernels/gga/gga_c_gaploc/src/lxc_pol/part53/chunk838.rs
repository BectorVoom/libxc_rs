//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 838/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk838<F: Float>(t2009: F, t2021: F, t47443: F, t47294: F, t7572: F, t7573: F, t12252: F, t2628: F, t1445: F, t47322: F, t807: F, t41411: F, t47130: F, t7290: F, t4820: F, t7513: F) -> (F, F, F, F, F, F, F) {
    let t47445 = t2021 * t47443 * t2009;
    let t47448 = t7572 * t7573 * t47294;
    let t47450 = t12252 * t2628;
    let t47462 = 0.23005755572352449806e1 * t807 * t1445 * t47322;
    let t47463 = 0.51123901271894332903e0 * t41411;
    let t47484 = t7290 * t47130;
    let t47486 = t7513 * t4820 * t47484;
    (t47445, t47448, t47450, t47462, t47463, t47484, t47486)
}
