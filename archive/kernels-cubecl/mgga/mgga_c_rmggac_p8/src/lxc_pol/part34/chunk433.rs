//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 433/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk433<F: Float>(t7844: F, t8642: F, t7785: F, t8646: F, t7788: F, t8650: F, t8626: F, t7829: F, t8632: F, t7782: F, t8636: F, t2392: F, t321: F) -> (F, F, F, F, F, F, F) {
    let t8889 = t7844 * t8642;
    let t8891 = t7785 * t8646;
    let t8893 = t7788 * t8650;
    let t8895 = t7785 * t8626;
    let t8897 = t7829 * t8632;
    let t8899 = t7782 * t8636;
    let t8901 = t2392 * t321;
    (t8889, t8891, t8893, t8895, t8897, t8899, t8901)
}
