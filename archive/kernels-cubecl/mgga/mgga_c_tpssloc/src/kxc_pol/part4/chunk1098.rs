//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1098/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1098<F: Float>(t17490: F, t17504: F, t17506: F, t17509: F, t17512: F, t17515: F, t17519: F, t17523: F, t17526: F, t17530: F, t17929: F, t17932: F, t17936: F, t17940: F, t17942: F, t17944: F, t17946: F, t17950: F, t17953: F, t17957: F) -> F {
    let t18174 = t17929 - t17932 - t17936 - t17490 + t17940 - t17942 - t17944 + t17946 + t17950 + t17953 - t17504 + t17506 + t17509 - t17512 - t17515 - t17519 + t17523 + t17526 + t17530 - t17957;
    t18174
}
