//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2555/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2555<F: Float>(t14085: F, t3053: F, t14080: F, t10936: F, t4669: F, t14077: F, t3103: F, t1036: F, t14114: F, t3082: F, t4617: F, t10904: F, t14025: F) -> (F, F, F, F, F, F, F) {
    let t49966 = t14085 * t3053;
    let t49972 = t14080 * t3053;
    let t49984 = t4669 * t10936;
    let t49987 = t14077 * t3103;
    let t49989 = t14114 * t1036;
    let t49993 = t4617 * t3082;
    let t50027 = t10904 * t14025;
    (t49966, t49972, t49984, t49987, t49989, t49993, t50027)
}
