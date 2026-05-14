//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 978/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk978<F: Float>(t3300: F, t39066: F, t1980: F, t7458: F, t1846: F, t7712: F, t1988: F, t9724: F, t2001: F, t5966: F, t1851: F, t7605: F, t5546: F, t1761: F, t30540: F, t2095: F) -> (F, F, F, F, F, F, F, F) {
    let t39271 = t3300 * t39066;
    let t39273 = t1980 * t7458 * t39271;
    let t39275 = t7712 * t1846;
    let t39277 = t1988 * t9724;
    let t39279 = t2001 * t5966;
    let t39281 = t7605 * t1851;
    let t39283 = t2001 * t5546;
    let t39285 = t30540 * t1761;
    let t39292 = t2095 * t39271;
    (t39273, t39275, t39277, t39279, t39281, t39283, t39285, t39292)
}
