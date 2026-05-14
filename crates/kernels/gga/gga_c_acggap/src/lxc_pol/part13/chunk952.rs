//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 952/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk952<F: Float>(t10146: F, t167: F, t576: F, t137: F, t3300: F, t4263: F, t30407: F, t31097: F, t495: F, t7325: F, t4410: F, t7561: F, t30543: F, t8610: F, t30934: F, t8614: F) -> (F, F, F, F, F) {
    let t34691 = t576 * t167 * t10146;
    let t34692 = t3300 * t137;
    let t34694 = t34691 * t34692 * t4263;
    let t34698 = t30407 * t31097 * t7325 * t495;
    let t34700 = t7561 * t4410;
    let t34702 = t30543 * t8610;
    let t34703 = 0.12862205435420921092e-1 * t34702;
    let t34704 = t30934 * t8614;
    (t34694, t34698, t34700, t34703, t34704)
}
