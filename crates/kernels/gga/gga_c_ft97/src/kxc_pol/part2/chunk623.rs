//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 623/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk623<F: Float>(t10969: F, t1853: F, t1904: F, t2983: F, t7793: F, t446: F, t3103: F, t358: F, t363: F, t1564: F, t1580: F, t3008: F, t1557: F, t942: F, t1559: F, t1882: F, t3010: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10970 = t10969 * t1853;
    let t10974 = t2983 * t1904;
    let t10975 = t7793 * t10974;
    let t10976 = t446 * t10975;
    let t10978 = t3103 * t358;
    let t10979 = t10978 * t363;
    let t10980 = t1564 * t10979;
    let t10981 = t446 * t10980;
    let t10983 = t3008 * t1580;
    let t10984 = t1564 * t10983;
    let t10985 = t446 * t10984;
    let t10987 = t942 * t1557;
    let t10988 = t10987 * t1559;
    let t10989 = t7793 * t10988;
    let t10990 = t446 * t10989;
    let t10992 = t1882 * t3010;
    (t10970, t10974, t10976, t10979, t10981, t10983, t10985, t10988, t10990, t10992)
}
