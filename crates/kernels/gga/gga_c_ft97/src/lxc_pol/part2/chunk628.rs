//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 628/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk628<F: Float>(t11013: F, t1866: F, t3281: F, t1755: F, t925: F, t1564: F, t446: F, t1882: F, t2989: F, t2994: F, t2985: F, t7775: F, t8192: F, t10976: F, t10981: F, t10985: F, t10990: F, t10993: F, t10996: F, t11000: F, t11005: F, t11010: F, t7778: F, t7782: F, t7820: F, t7822: F) -> (F, F, F, F, F, F, F) {
    let t11014 = t1866 * t11013;
    let t11015 = t3281 * t11014;
    let t11017 = t925 * t1755;
    let t11018 = t1564 * t11017;
    let t11019 = t446 * t11018;
    let t11021 = t1882 * t2989;
    let t11022 = t11021 / 27.0;
    let t11023 = t1882 * t2994;
    let t11024 = 2.0 / 27.0 * t11023;
    let t11025 = t1882 * t2985;
    let t11026 = 2.0 / 81.0 * t11025;
    let t11027 = 4.0 / 81.0 * t7775;
    let t11031 = 4.0 / 27.0 * t8192;
    let t11032 = -t7822 / 27.0 + 2.0 / 27.0 * t10976 + t10981 / 9.0 + t10985 / 18.0 + t10990 / 27.0 - t10993 + 2.0 / 9.0 * t10996 + t11000 / 9.0 + 4.0 / 9.0 * t11005 - 5.0 / 81.0 * t11010 - 4.0 / 27.0 * t11015 + t11019 / 18.0 - t11022 - t11024 + t11026 - t11027 + t7778 / 54.0 + t7782 / 81.0 - t7820 / 27.0 - t11031;
    (t11015, t11017, t11019, t11021, t11023, t11025, t11032)
}
