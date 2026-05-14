//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1036/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1036<F: Float>(t11438: F, t21649: F, t3021: F, t1649: F, t33303: F, t5553: F, t27149: F, t520: F, t9061: F, t11449: F, t11451: F, t1803: F, t190: F, t21183: F, t11492: F, t34675: F) -> (F, F, F, F, F, F) {
    let t34791 = t11438 * t3021 * t21649;
    let t34793 = t33303 * t1649;
    let t34794 = t5553 * t34793;
    let t34797 = t9061 * t520 * t27149;
    let t34802 = t1803 * t190 * t11449 * t11451 * t21183;
    let t34804 = t34675 * t11492;
    (t34791, t34793, t34794, t34797, t34802, t34804)
}
