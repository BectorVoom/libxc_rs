//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1227/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1227<F: Float>(t10170: F, t107: F, t1415: F, t1417: F, t30805: F, t30808: F, t30810: F, t30813: F, t30821: F, t30824: F, t30828: F, t30833: F, t34592: F, t34595: F, t34603: F, t34607: F, t34609: F, t34612: F, t34614: F) -> (F,) {
    let t34615 = -t34592 + t34595 + t30805 - t30808 + 0.79445533226334281486e-1 * t1415 * t10170 * t107 * t1417 + t30810 + t30813 + t30821 - t30824 - t30828 - t30833 - t34603 - t34607 - t34609 - t34612 - t34614;
    (t34615,)
}
