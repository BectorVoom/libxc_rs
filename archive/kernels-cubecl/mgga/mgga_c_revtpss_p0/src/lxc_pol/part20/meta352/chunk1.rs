//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1283/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1283<F: Float>(t39436: F, t10578: F, t9863: F, t762: F, t9291: F, t2629: F, t2251: F) -> (F, F, F, F, F) {
    let t39437 = F::cast_from(0.65061487801810439052e-1_f64) * t39436;
    let t39438 = t10578 * t9863;
    let t39439 = F::cast_from(0.65061487801810439052e-1_f64) * t39438;
    let t39440 = t9291 * t762;
    let t39442 = F::cast_from(0.67471172535210825684e-1_f64) * t2629 * t39440;
    let t39443 = t2251 * t2251;
    (t39437, t39439, t39440, t39442, t39443)
}
