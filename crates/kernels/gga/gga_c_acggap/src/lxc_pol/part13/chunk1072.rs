//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1072/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1072<F: Float>(t30543: F, t8610: F, t30934: F, t8614: F, t1181: F, t4533: F, t604: F, t7575: F, t7433: F, t8522: F, t8518: F, t5012: F, t7564: F, t8600: F) -> (F, F, F, F, F, F) {
    let t34702 = t30543 * t8610;
    let t34703 = F::new(0.12862205435420921092e-1) * t34702;
    let t34704 = t30934 * t8614;
    let t34708 = t7575 * t1181 * t604 * t4533;
    let t34710 = t7433 * t8522;
    let t34711 = F::new(0.12862205435420921092e-2) * t34710;
    let t34712 = t7433 * t8518;
    let t34713 = F::new(0.12862205435420921092e-2) * t34712;
    let t34716 = t7564 * t1181 * t8600 * t5012;
    (t34703, t34704, t34708, t34711, t34713, t34716)
}
