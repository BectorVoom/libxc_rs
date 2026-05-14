//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 840/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk840<F: Float>(t3178: F, t6492: F, t1092: F, t1773: F, t4772: F, t1131: F, t1096: F, t1023: F, t18463: F, t1020: F, t6689: F, t922: F, t9410: F, t3200: F, t2861: F, t6560: F) -> (F, F, F, F, F, F, F) {
    let t19627 = t3178 * t6492;
    let t19628 = t1092 * t19627;
    let t19630 = t4772 * t1773;
    let t19631 = t1131 * t19630;
    let t19632 = t1096 * t19631;
    let t19633 = t1092 * t19632;
    let t19635 = t18463 * t1023;
    let t19636 = t1020 * t19635;
    let t19638 = t6689 * t922;
    let t19639 = t9410 * t19638;
    let t19640 = t3200 * t19639;
    let t19642 = t2861 * t6560;
    (t19628, t19630, t19633, t19636, t19638, t19640, t19642)
}
