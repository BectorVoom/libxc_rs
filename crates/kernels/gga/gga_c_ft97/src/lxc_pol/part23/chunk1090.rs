//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1090/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1090<F: Float>(t2393: F, t420: F, t703: F, t3789: F, t41547: F, t6: F, t8: F, t24322: F, t444: F, t3771: F, t6032: F, t93076: F, t1410: F, t9681: F, t6044: F, t625: F) -> (F, F, F, F, F, F, F) {
    let t96424 = t420 * t703 * t2393;
    let t96442 = t3789 * t41547 * t6 * t8;
    let t96450 = t24322 * t444;
    let t96451 = t3789 * t96450;
    let t96465 = t3771 * t6032 * t93076;
    let t96510 = t9681 * t1410;
    let t96535 = t6044 * t625;
    (t96424, t96442, t96450, t96451, t96465, t96510, t96535)
}
