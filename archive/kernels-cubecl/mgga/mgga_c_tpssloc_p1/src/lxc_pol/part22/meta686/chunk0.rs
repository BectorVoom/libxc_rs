//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2263/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2263<F: Float>(t3400: F, t6063: F, t1098: F, t18245: F, t3312: F, t5983: F, t18496: F, t699: F, t18517: F, t18514: F, t18520: F, t2403: F, t6011: F) -> (F, F, F, F, F, F, F, F) {
    let t63602 = t6063 * t3400;
    let t63750 = t18245 * t1098;
    let t63755 = t5983 * t3312;
    let t63841 = t699 * t18496;
    let t63843 = t699 * t18517;
    let t63845 = t699 * t18514;
    let t63886 = t699 * t18520;
    let t63888 = t2403 * t6011;
    (t63602, t63750, t63755, t63841, t63843, t63845, t63886, t63888)
}
