//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2463/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2463<F: Float>(t1427: F, t1903: F, t22: F, t9647: F, t2453: F, t3908: F, t5711: F, t14296: F, t9303: F, t213: F, t556: F, t9656: F) -> (F, F, F, F, F) {
    let t47781 = t9647 * t1427 * t1903 * t22;
    let t47784 = t2453 * t5711 * t3908;
    let t47785 = F::cast_from(0.34697458558045176417e-2_f64) * t47784;
    let t47786 = t9303 * t14296;
    let t47793 = t213 * t556;
    let t47794 = t9656 * t1903;
    (t47781, t47785, t47786, t47793, t47794)
}
