//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 432/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk432<F: Float>(t86: F, t112: F, t18: F, t113: F, t1577: F, t3297: F, t5: F, t502: F, t505: F, t989: F, t992: F, t1022: F, t1952: F, t1546: F, t89: F, t998: F) -> (F, F, F) {
    let t87 = 10000000.0 <= t86;
    let t3307 = t112 * t18;
    let t3312 = piecewise3(t87, 0.0, t5 * t3297 * t113 / 4.0 + t5 * t989 * t505 / 4.0 + t5 * t502 * t992 / 4.0 - t5 * t3307 * t1577 / 2.0);
    let t3313 = t1952 * t1022;
    let t3318 = t89 * t1546 * t998;
    (t3312, t3313, t3318)
}
