//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1400/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1400<F: Float>(t114373: F, t114375: F, t126877: F, t126880: F, t126882: F, t126886: F, t126890: F, t126894: F, t126897: F, t126902: F, t126907: F, t126910: F, t113379: F, t113386: F, t114384: F, t126913: F, t126915: F, t126919: F, t126923: F, t126927: F, t126929: F, t126932: F, t126935: F, t126938: F) -> (F, F) {
    let t128223 = -t114373 - t114375 - t126877 / 36.0 + t126880 / 18.0 - 2.0 / 27.0 * t126882 - 2.0 / 9.0 * t126886 + t126890 / 9.0 + 2.0 / 27.0 * t126894 - 2.0 / 9.0 * t126897 + t126902 / 6.0 + t126907 / 12.0 - 2.0 / 9.0 * t126910;
    let t128235 = t114384 - t126913 / 54.0 - t126915 / 81.0 + t126919 / 18.0 + t126923 / 18.0 + t126927 / 27.0 - t126929 / 27.0 + 2.0 / 3.0 * t126932 + 4.0 / 9.0 * t126935 - 4.0 / 27.0 * t126938 + 8.0 / 27.0 * t113379 + 2.0 / 27.0 * t113386;
    (t128223, t128235)
}
