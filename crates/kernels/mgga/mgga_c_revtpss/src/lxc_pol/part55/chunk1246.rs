//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1246/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1246<F: Float>(t125584: F, t120977: F, t120983: F, t122284: F, t122288: F, t122290: F, t122297: F, t122299: F, t125590: F, t1444: F, t1903: F, t32250: F, t32673: F, t34226: F, t8706: F) -> F {
    let t128595 = F::new(0.13223814266738539448e-3) * t125584;
    let t128609 = t128595 - F::new(0.25702851531048074406e-1) * t122284 - F::new(0.14279934416275588154e-1) * t122288 + F::new(0.25389723392137995738e-1) * t122290 - F::new(0.17135921299530705785e1) * t8706 * t32250 * t32673 * t1903 - F::new(0.17135921299530705785e1) * t8706 * t32250 * t34226 * t1444 + t122297 - F::new(0.14279934416275588154e-1) * t122299 - t120977 - F::new(0.29749863367240808656e-2) * t125590 - t120983;
    t128609
}
