//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 587/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk587(t17807: f64, t17819: f64, t17843: f64, t232: f64, t24289: f64, t24346: f64, t27506: f64, t27529: f64, t27558: f64, t27562: f64, t27566: f64, t27570: f64, t27576: f64, t27579: f64, t27582: f64, t27584: f64, t27588: f64, t27596: f64, t27601: f64, t27605: f64, t27609: f64, t3762: f64, t3774: f64, t3786: f64, t6023: f64, t6043: f64, t6045: f64, t6046: f64) -> f64 {
    let t27613 = -0.51690243689028715488e-5_f64 * t3774 * t27558 - 0.1721820212247325051e-5_f64 * t3774 * t27562 - 0.13784064983740990796e-3_f64 * t27566 * t17843 - 0.21281202793209876543e-2_f64 * t27570 + 0.23254900946437792e-1_f64 * t24346 * t3786 + t24289 - 0.15137014751914110597e-3_f64 * t27576 + 0.12768721675925925926e-1_f64 * t27579 - 0.12768721675925925926e-1_f64 * t27582 - 0.13784064983740990796e-3_f64 * t3774 * t27584 * t3762 + 0.38306165027777777778e-1_f64 * t6043 * t6045 * t27588 - 0.10214977340740740741e0_f64 * t6043 * t27506 * t6046 + 0.25845121844514357744e-4_f64 * t3774 * t6023 * t27596 - 0.60102574844279699039e-6_f64 * t17819 * t27601 + 0.61277550024922479209e-6_f64 * t17807 * t27605 * t3762 + 0.44540303667943584666e-3_f64 * t27609 * t232 * t27529;
    t27613
}
