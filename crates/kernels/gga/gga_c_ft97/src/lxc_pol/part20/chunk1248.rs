//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1248/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1248<F: Float>(t112816: F, t24981: F, t28755: F, t113152: F, t28772: F, t25162: F, t28764: F, t28769: F, t28773: F, t113126: F, t24976: F, t6317: F, t113543: F, t113546: F, t113549: F, t113553: F, t113556: F, t99809: F) -> (F, F, F, F, F, F, F) {
    let t113559 = t28755 * t24981 * t112816;
    let t113562 = t28755 * t28772 * t113152;
    let t113564 = t25162 * t28764;
    let t113565 = 2.0 / 9.0 * t113564;
    let t113566 = t25162 * t28769;
    let t113567 = 2.0 / 9.0 * t113566;
    let t113568 = t25162 * t28773;
    let t113569 = 2.0 / 27.0 * t113568;
    let t113571 = t6317 * t24976 * t113126;
    let t113573 = 2.0 * t113543 - 2.0 / 3.0 * t113546 + 2.0 / 9.0 * t113549 - t113553 / 6.0 + 4.0 / 3.0 * t113556 + 4.0 / 3.0 * t113559 - 4.0 / 9.0 * t113562 + t113565 + t113567 - t113569 - 2.0 / 3.0 * t113571 - t99809;
    (t113559, t113562, t113564, t113566, t113568, t113571, t113573)
}
