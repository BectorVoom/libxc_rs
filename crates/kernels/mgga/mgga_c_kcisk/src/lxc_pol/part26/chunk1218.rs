//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1218/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1218<F: Float>(t1556: F, t27682: F, t27934: F, t4350: F, t1458: F, t27045: F, t2168: F, t6211: F, t220: F, t1322: F, t8048: F, t7710: F, t31861: F, t31863: F, t31865: F, t31875: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t84770 = t27682 * t1556;
    let t84866 = t27934 * t4350;
    let t85381 = t27045 * t1458;
    let t87991 = t2168 * t6211;
    let t88000 = t2168 * t220;
    let t88072 = t8048 * t1322;
    let t88147 = t7710 * t1322;
    let t109134 = 3.0 * t31861;
    let t109135 = 12.0 * t31863;
    let t109136 = 6.0 * t31865;
    let t109141 = 12.0 * t31875;
    (t84770, t84866, t85381, t87991, t88000, t88072, t88147, t109134, t109135, t109136, t109141)
}
