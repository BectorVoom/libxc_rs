//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 585/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk585<F: Float>(t28128: F, t3864: F, t14127: F, t14159: F, t6075: F, t24737: F, t3859: F, t13885: F, t255: F, t9707: F, t3837: F, t6074: F, t3842: F, t1901: F, t24567: F, t28102: F, t28106: F, t28110: F, t28113: F, t28116: F, t28120: F, t28125: F, t446: F) -> (F, F, F, F, F, F) {
    let t28129 = t28128 * t3864;
    let t28130 = t14127 * t28129;
    let t28133 = t14159 * t6075;
    let t28136 = t24737 * t3859;
    let t28137 = t13885 * t28136;
    let t28140 = t9707 * t255;
    let t28141 = t6074 * t3837;
    let t28142 = t28140 * t28141;
    let t28145 = t24737 * t3842;
    let t28146 = t13885 * t28145;
    let t28149 = -t446 * t28102 / 3.0 - 2.0 / 9.0 * t24567 + t28106 / 9.0 + t446 * t28110 / 3.0 - t28113 / 9.0 - t446 * t28116 / 3.0 - t446 * t28120 / 3.0 + t1901 * t28125 / 9.0 - 2.0 / 3.0 * t1901 * t28130 + t1901 * t28133 / 9.0 - 2.0 / 3.0 * t1901 * t28137 - 2.0 * t1901 * t28142 - 2.0 / 3.0 * t1901 * t28146;
    (t28129, t28136, t28140, t28141, t28145, t28149)
}
