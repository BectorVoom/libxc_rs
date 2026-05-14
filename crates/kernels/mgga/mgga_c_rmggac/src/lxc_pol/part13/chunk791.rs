//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 791/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk791<F: Float>(t39700: F, t903: F, t1679: F, t7203: F, t7206: F, t7255: F, t8422: F, t2289: F, t35384: F, t1986: F, t5142: F, t675: F, t7944: F, t1971: F, t27326: F, t3351: F, t7262: F) -> (F, F, F, F, F, F, F) {
    let t39701 = t903 * t39700;
    let t39705 = t1679 * t7203;
    let t39706 = t39705 * t7206;
    let t39709 = t7255 * t8422;
    let t39711 = t35384 * t2289;
    let t39715 = t675 * t1986 * t5142;
    let t39717 = t7944 * t2289;
    let t39721 = t3351 * t1971 * t7262 * t27326;
    (t39701, t39706, t39709, t39711, t39715, t39717, t39721)
}
