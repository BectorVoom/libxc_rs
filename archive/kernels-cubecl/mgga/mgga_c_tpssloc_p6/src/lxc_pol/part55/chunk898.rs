//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 898/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk898<F: Float>(t1222: F, t7334: F, t2141: F, t3540: F, t3: F, t7324: F, t1184: F, t52: F, t460: F, t3548: F, t7310: F, t2127: F, t3545: F) -> (F, F, F, F, F, F, F) {
    let t24675 = t7334 * t1222;
    let t24681 = t2141 * t3540 / F::cast_from(6912.0_f64);
    let t24682 = t7324 * t3;
    let t24683 = t52 * t1184;
    let t24684 = t24683 * t460;
    let t24685 = t24682 * t24684;
    let t24690 = t7310 * t3548;
    let t24704 = t2127 * t3545 / F::cast_from(432.0_f64);
    (t24675, t24681, t24682, t24683, t24685, t24690, t24704)
}
