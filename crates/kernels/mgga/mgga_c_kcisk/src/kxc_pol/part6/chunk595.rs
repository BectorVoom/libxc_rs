//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 595/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk595<F: Float>(t645: F, t1755: F, t8786: F, t2436: F, t2442: F, t340: F, t639: F, t642: F, t8773: F, t8781: F, t655: F, t2364: F, t2464: F, t5015: F, t5020: F, t7715: F, t1775: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t646 = t645 < -0.66725e-1;
    let t8787 = t1755 * t8786;
    let t8792 = piecewise3(t646, 0.0, 10.0 / 9.0 * t340 * t8773 * t642 - 20.0 / 27.0 * t340 * t2436 * t2442 + 40.0 / 81.0 * t340 * t639 * t8781 - 10.0 / 27.0 * t340 * t639 * t8787);
    let t8793 = t8792 * sigma2;
    let t8794 = t8793 * t655;
    let t8797 = t2364 * t2464;
    let t8798 = t5015 * t8797;
    let t8801 = t5020 * t7715;
    let t8802 = t1775 * t8801;
    (t8787, t8793, t8794, t8797, t8798, t8801, t8802)
}
