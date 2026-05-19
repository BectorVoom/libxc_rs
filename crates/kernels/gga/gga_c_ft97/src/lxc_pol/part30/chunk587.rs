//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 587/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk587<F: Float>(t17807: F, t17819: F, t17843: F, t232: F, t24289: F, t24346: F, t27506: F, t27529: F, t27558: F, t27562: F, t27566: F, t27570: F, t27576: F, t27579: F, t27582: F, t27584: F, t27588: F, t27596: F, t27601: F, t27605: F, t27609: F, t3762: F, t3774: F, t3786: F, t6023: F, t6043: F, t6045: F, t6046: F) -> F {
    let t27613 = -F::cast_from(0.51690243689028715488e-5_f64) * t3774 * t27558 - F::cast_from(0.1721820212247325051e-5_f64) * t3774 * t27562 - F::cast_from(0.13784064983740990796e-3_f64) * t27566 * t17843 - F::cast_from(0.21281202793209876543e-2_f64) * t27570 + F::cast_from(0.23254900946437792e-1_f64) * t24346 * t3786 + t24289 - F::cast_from(0.15137014751914110597e-3_f64) * t27576 + F::cast_from(0.12768721675925925926e-1_f64) * t27579 - F::cast_from(0.12768721675925925926e-1_f64) * t27582 - F::cast_from(0.13784064983740990796e-3_f64) * t3774 * t27584 * t3762 + F::cast_from(0.38306165027777777778e-1_f64) * t6043 * t6045 * t27588 - F::cast_from(0.10214977340740740741e0_f64) * t6043 * t27506 * t6046 + F::cast_from(0.25845121844514357744e-4_f64) * t3774 * t6023 * t27596 - F::cast_from(0.60102574844279699039e-6_f64) * t17819 * t27601 + F::cast_from(0.61277550024922479209e-6_f64) * t17807 * t27605 * t3762 + F::cast_from(0.44540303667943584666e-3_f64) * t27609 * t232 * t27529;
    t27613
}
