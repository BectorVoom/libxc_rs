//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1832/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1832<F: Float>(t14255: F, t291: F, t10629: F, t1580: F, t10632: F, t2906: F, t959: F, t1573: F, t2904: F, t4408: F, t923: F, t1561: F, t2885: F) -> (F, F, F, F, F, F, F, F) {
    let t14257 = F::cast_from(0.621814e-1_f64) * t14255 * t291;
    let t14258 = t10629 * t1580;
    let t14259 = t10632 * t2906;
    let t14260 = t14258 * t14259;
    let t14262 = F::cast_from(0.10254018858216406658e4_f64) * t959 * t14260;
    let t14263 = t1573 * t2904;
    let t14266 = t4408 * t923;
    let t14271 = t1561 * t2885;
    (t14257, t14258, t14259, t14260, t14262, t14263, t14266, t14271)
}
