//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1387/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1387<F: Float>(t10318: F, t1646: F, t4398: F, t26609: F, t6628: F, t10170: F, t107: F, t1415: F, t1417: F, t30805: F, t30808: F, t30810: F, t30813: F, t30821: F, t30824: F, t30828: F, t30833: F, t34592: F, t34595: F, t34603: F, t34607: F, t34609: F) -> F {
    let t34612 = F::cast_from(0.71500979903700853338e0_f64) * t4398 * t10318 * t1646;
    let t34614 = F::cast_from(0.21450293971110256002e1_f64) * t26609 * t6628;
    let t34615 = -t34592 + t34595 + t30805 - t30808 + F::cast_from(0.79445533226334281486e-1_f64) * t1415 * t10170 * t107 * t1417 + t30810 + t30813 + t30821 - t30824 - t30828 - t30833 - t34603 - t34607 - t34609 - t34612 - t34614;
    t34615
}
