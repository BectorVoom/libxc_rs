//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1357/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1357<F: Float>(t105894: F, t105882: F, t105884: F, t105888: F, t105891: F, t105899: F, t105903: F, t105907: F, t105912: F, t95320: F, t95322: F, t95330: F, t105941: F, t105919: F, t105926: F, t105930: F, t105935: F, t105940: F, t105945: F, t95356: F, t95370: F, t95389: F, t96140: F, t96143: F) -> (F, F) {
    let t106144 = 4.0 / 9.0 * t105894;
    let t106152 = -t105882 / 6.0 - 22.0 / 27.0 * t105884 - t105888 / 36.0 - 4.0 / 9.0 * t105891 - t106144 + t95320 / 54.0 + 2.0 / 27.0 * t95322 - t105899 / 3.0 - t105903 / 27.0 - 4.0 / 27.0 * t105907 + t105912 / 9.0 - 4.0 / 27.0 * t95330;
    let t106160 = t105941 / 27.0;
    let t106163 = t105919 / 2.0 - 2.0 / 9.0 * t95356 + t105926 / 12.0 + t105930 / 9.0 + t96140 - t95370 / 27.0 + t105935 / 12.0 + t105940 / 3.0 + t96143 - t106160 - 2.0 / 81.0 * t95389 + t105945 / 27.0;
    (t106152, t106163)
}
