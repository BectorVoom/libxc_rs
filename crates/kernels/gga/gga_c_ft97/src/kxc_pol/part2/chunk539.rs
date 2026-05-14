//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 539/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk539<F: Float>(t2347: F, t258: F, t3886: F, t3891: F, t1131: F, t684: F, t2599: F, t1154: F, t2475: F, t747: F, t91: F, t1148: F, t1775: F, t2: F, t2486: F, t3691: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3892 = t258 * t2347;
    let t3893 = t3892 * t3886;
    let t3894 = t3891 * t3893;
    let t3897 = t258 * t1131;
    let t3898 = t3897 * t684;
    let t3899 = t2599 * t3898;
    let t3902 = t2475 * t1154;
    let t3904 = t91 * t3902 * t747;
    let t3908 = t1775 * t1148;
    let t3910 = t2486 * t2;
    let t3911 = t3910 * t3691;
    (t3892, t3893, t3894, t3897, t3898, t3899, t3902, t3904, t3908, t3910, t3911)
}
