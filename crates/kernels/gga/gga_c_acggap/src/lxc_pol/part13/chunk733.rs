//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 733/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk733<F: Float>(t2079: F, t7724: F, t599: F, t930: F, t1181: F, t2068: F, t121: F, t939: F, t382: F, t151: F, t947: F, t1004: F, t1997: F) -> (F, F, F, F, F, F, F, F) {
    let t7725 = t2079 * t7724;
    let t7726 = F::new(0.10718504529517434243e-3) * t7725;
    let t7727 = t599 * t930;
    let t7728 = t1181 * t7727;
    let t7729 = t2068 * t7728;
    let t7731 = t939 * t121;
    let t7732 = t7731 * t382;
    let t7733 = t151 * t7732;
    let t7734 = t7733 * t947;
    let t7736 = t1004 * t1997;
    (t7726, t7727, t7728, t7729, t7731, t7732, t7734, t7736)
}
