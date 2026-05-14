//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 866/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk866<F: Float>(t17807: F, t17819: F, t17843: F, t232: F, t24289: F, t24346: F, t27506: F, t27529: F, t27558: F, t27562: F, t27566: F, t27570: F, t27576: F, t27579: F, t27582: F, t27584: F, t27588: F, t27596: F, t27601: F, t27605: F, t27609: F, t3762: F, t3774: F, t3786: F, t6023: F, t6043: F, t6045: F, t6046: F) -> (F,) {
    let t27613 = -0.51690243689028715488e-5 * t3774 * t27558 - 0.1721820212247325051e-5 * t3774 * t27562 - 0.13784064983740990796e-3 * t27566 * t17843 - 0.21281202793209876543e-2 * t27570 + 0.23254900946437792e-1 * t24346 * t3786 + t24289 - 0.15137014751914110597e-3 * t27576 + 0.12768721675925925926e-1 * t27579 - 0.12768721675925925926e-1 * t27582 - 0.13784064983740990796e-3 * t3774 * t27584 * t3762 + 0.38306165027777777778e-1 * t6043 * t6045 * t27588 - 0.10214977340740740741e0 * t6043 * t27506 * t6046 + 0.25845121844514357744e-4 * t3774 * t6023 * t27596 - 0.60102574844279699039e-6 * t17819 * t27601 + 0.61277550024922479209e-6 * t17807 * t27605 * t3762 + 0.44540303667943584666e-3 * t27609 * t232 * t27529;
    (t27613,)
}
