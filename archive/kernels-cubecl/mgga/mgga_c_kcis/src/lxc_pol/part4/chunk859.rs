//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 859/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk859<F: Float>(t1468: F, t5672: F, t1464: F, t1489: F, t2001: F, t1396: F, t4123: F, t1948: F, t4142: F, t1014: F, t2007: F, t1984: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5673 = t1468 * t5672;
    let t5674 = t1464 * t5673;
    let t5676 = t2001 * t1489;
    let t5677 = t1396 * t5676;
    let t5678 = t4123 * t5677;
    let t5679 = t1464 * t5678;
    let t5681 = t4142 * t1948;
    let t5684 = t1014 * t2007;
    let t5686 = t1014 * t1984;
    (t5673, t5674, t5676, t5677, t5678, t5679, t5681, t5684, t5686)
}
