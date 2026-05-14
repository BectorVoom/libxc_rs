//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1109/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1109<F: Float>(t13748: F, t13750: F, t13754: F, t13771: F, t13814: F, t16233: F, t16238: F, t16241: F, t16244: F, t16249: F, t16253: F, t16255: F, t16264: F, t16274: F, t16288: F, t16292: F, t16294: F, t16300: F, t16304: F, t21737: F, t21740: F, t21743: F, t21745: F, t21747: F, t21751: F, t21755: F, t21759: F, t21762: F) -> (F, F) {
    let t22590 = -4.0 / 3.0 * t16233 - t16238 / 2.0 + t16241 / 3.0 + t16244 / 6.0 - 5.0 / 3.0 * t16249 + 56.0 / 9.0 * t16253 + 4.0 * t16255 - 4.0 / 3.0 * t16264 - 8.0 / 3.0 * t16274 - t13814 + 140.0 / 27.0 * t13748 + 14.0 / 9.0 * t13750 - 7.0 / 9.0 * t13754 - 5.0 / 3.0 * t13771;
    let t22605 = -t21737 / 2.0 + t21740 / 3.0 + t21743 / 6.0 - t21745 / 3.0 + t21747 / 6.0 + t21751 / 6.0 - t21755 / 12.0 - t21759 / 12.0 - 6.0 * t21762 + 4.0 / 3.0 * t16288 - t16292 / 3.0 - 40.0 / 27.0 * t16294 + 2.0 / 3.0 * t16300 - t16304 / 12.0;
    (t22590, t22605)
}
