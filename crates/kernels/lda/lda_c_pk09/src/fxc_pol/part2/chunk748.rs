//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 748/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk748<F: Float>(t2305: F, t569: F, t3254: F, t7608: F, t155: F, t7693: F, t143: F, t1091: F, t2314: F, t8069: F, t890: F, t8073: F, t2210: F, t3273: F, t3826: F, t3829: F, t4528: F, t4530: F, t4531: F, t7598: F, t7602: F) -> (F, F, F) {
    let t8577 = t2305 * t569;
    let t8585 = t3254 * t7608;
    let t8587 = t155 * t7693;
    let t8589 = t143 * t7693;
    let t8592 = t2314 * t1091;
    let t8595 = t890 * t8069;
    let t8597 = t890 * t8073;
    let t8599 = -t4528 + t4530 + 0.8357942709722364 * t8577 + 38.978347549160304 * t3826 * t7598 + 19.489173774580152 * t3826 * t7602 - 19.489173774580152 * t3829 * t2210 + 19.489173774580152 * t8585 - 12.992782516386768 * t8587 + 2.507382812916709 * t8589 - 1.2536914064583544 * t4531 + 2.0 * t3273 * t8592 + 3.600163427964126 * t8595 + 3.600163427964126 * t8597;
    (t8595, t8597, t8599)
}
