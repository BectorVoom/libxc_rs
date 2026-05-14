//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 881/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk881<F: Float>(t24412: F, t3864: F, t681: F, t6843: F, t1403: F, t1137: F, t1427: F, t1454: F, t24179: F, t27890: F, t27894: F, t27897: F, t27899: F, t27908: F, t27911: F, t3683: F, t3827: F, t5996: F, t6064: F, t6192: F, t6745: F, t6840: F) -> (F, F, F) {
    let t27913 = t24412 * t3864;
    let t27915 = t681 * t6843;
    let t27916 = t1403 * t27915;
    let t27921 = -2.0 * t27890 + t27894 * t1427 / 6.0 - 2.0 * t27897 - 2.0 * t27899 - t3827 * t1454 - t1137 * t6192 - t3683 * t1454 + t5996 * t6840 / 6.0 + t1403 * t27908 / 6.0 + 4.0 * t27911 + 4.0 * t27913 - t27916 / 18.0 + t6745 * t6064 / 6.0 + t24179 / 9.0;
    (t27913, t27915, t27921)
}
