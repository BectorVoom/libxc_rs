//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 923/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk923<F: Float>(t1445: F, t2642: F, t1441: F, t833: F, t2645: F, t532: F, t160: F, t450: F, t4075: F, t743: F, t4078: F, t4083: F, t733: F, t4086: F, t4096: F, t1431: F, t2466: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11958 = t1445 * t2642;
    let t11960 = t1441 * t833;
    let t11962 = t532 * t2645;
    let t11966 = t160 * t450;
    let t11967 = 0.71734315950379065738e-1 * t11966;
    let t11974 = t743 * t4075;
    let t11977 = t743 * t4078;
    let t11985 = t733 * t4083;
    let t11987 = t733 * t4086;
    let t11995 = t733 * t4096;
    let t12003 = t2466 * t1431;
    (t11958, t11960, t11962, t11966, t11967, t11974, t11977, t11985, t11987, t11995, t12003)
}
