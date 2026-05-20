//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2465/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2465<F: Float>(t47837: F, t10069: F, t13731: F, t137: F, t14103: F, t47480: F, t9675: F, t14099: F, t2453: F, t9676: F, t14109: F, t9680: F, t9685: F) -> (F, F, F, F, F, F) {
    let t47838 = F::cast_from(0.34697458558045176417e-2_f64) * t47837;
    let t47839 = t10069 * t13731;
    let t47844 = t47480 * t14103 * t137 * t9675;
    let t47845 = F::cast_from(0.69394917116090352834e-2_f64) * t47844;
    let t47856 = t2453 * t14099;
    let t47857 = t47856 * t9676;
    let t47858 = F::cast_from(0.34697458558045176417e-2_f64) * t47857;
    let t47860 = t9680 * t14109 * t9685;
    (t47838, t47839, t47845, t47856, t47858, t47860)
}
