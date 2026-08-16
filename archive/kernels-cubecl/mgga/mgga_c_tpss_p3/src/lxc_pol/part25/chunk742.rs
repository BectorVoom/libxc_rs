//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 742/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk742<F: Float>(t4891: F, t885: F, t2557: F, t2564: F, t3746: F, t3795: F, t4828: F, t4832: F, t4836: F, t4848: F, t4855: F, t4861: F, t4863: F, t4867: F, t4870: F, t4873: F) -> (F, F) {
    let t4892 = t4891 * t885;
    let t4907 = -F::cast_from(0.17648625e1_f64) * t4848 + F::cast_from(0.3529725e1_f64) * t4855 + t2557 + F::cast_from(0.34431666666666666666e0_f64) * t3746 - F::cast_from(0.34431666666666666667e0_f64) * t4828 + F::cast_from(0.103295e1_f64) * t4832 - F::cast_from(0.516475e0_f64) * t4836 + F::cast_from(0.31558125e0_f64) * t4861 + F::cast_from(0.6311625e0_f64) * t4863 + t2564 + F::cast_from(0.13892666666666666667e0_f64) * t3795 - F::cast_from(0.34731666666666666667e-1_f64) * t4867 + F::cast_from(0.20839e0_f64) * t4870 - F::cast_from(0.104195e0_f64) * t4873;
    (t4892, t4907)
}
