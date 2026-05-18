//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1015/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1015<F: Float>(t33994: F, t7839: F, t8779: F, t4991: F, t7822: F, t5192: F, t2068: F, t4680: F, t8778: F, t2001: F, t5014: F, t1089: F, t535: F, t7553: F, t7554: F) -> (F, F, F, F, F, F, F) {
    let t33995 = F::new(0.7145669686344956162e-3) * t33994;
    let t33996 = t7839 * t8779;
    let t33997 = F::new(0.42874018118069736972e-3) * t33996;
    let t33998 = t7822 * t4991;
    let t34000 = t7822 * t5192;
    let t34003 = t2068 * t4680 * t8778;
    let t34005 = t2001 * t5014;
    let t34009 = t7553 * t1089 * t535 * t7554;
    (t33995, t33997, t33998, t34000, t34003, t34005, t34009)
}
