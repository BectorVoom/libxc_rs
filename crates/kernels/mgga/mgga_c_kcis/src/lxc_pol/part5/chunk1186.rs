//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1186/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1186<F: Float>(t1466: F, t7192: F, t1490: F, t1464: F, t1498: F, t20961: F, t3728: F, t6924: F, t6929: F, t3738: F, t7203: F, t10443: F, t18431: F, t19653: F, t8: F, t1495: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t21955 = t7192 * t1466;
    let t21956 = t21955 * sigma2;
    let t21957 = t21956 * t1490;
    let t21958 = t1464 * t21957;
    let t21960 = t20961 * t1498;
    let t21961 = t1464 * t21960;
    let t21963 = t3728 * t6924;
    let t21965 = t3728 * t6929;
    let t21967 = t3738 * t7203;
    let t21968 = t1464 * t21967;
    let t21971 = t18431 * t8 - t10443 - t19653;
    let t21972 = t1495 * t21971;
    (t21955, t21958, t21961, t21963, t21965, t21968, t21971, t21972)
}
