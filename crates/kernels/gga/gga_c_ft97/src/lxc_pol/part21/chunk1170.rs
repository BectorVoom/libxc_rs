//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1170/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1170<F: Float>(t116661: F, t1871: F, t22952: F, t432: F, t4533: F, t5675: F, t25528: F, t28: F, t3103: F, t89: F, t101691: F, t102209: F, t116635: F, t116638: F, t116642: F, t116646: F, t116650: F, t116655: F, t116659: F) -> (F, F, F) {
    let t116662 = t116661 / 3.0;
    let t116666 = t22952 * t1871 * t5675 * t4533 * t432;
    let t116670 = t89 * t28 * t25528 * t3103;
    let t116672 = 8.0 / 9.0 * t116635 - 4.0 / 3.0 * t116638 - t116642 + 4.0 / 27.0 * t101691 - t116646 + 15.0 / 16.0 * t116650 + 3.0 / 4.0 * t116655 - 3.0 / 4.0 * t116659 + t116662 - t116666 / 2.0 + 4.0 * t116670 - t102209;
    (t116666, t116670, t116672)
}
