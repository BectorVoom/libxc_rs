//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 879/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk879(t17667: f64, t637: f64, t639: f64, t12132: f64, t12143: f64, t12162: f64, t12164: f64, t12165: f64, t12171: f64, t12174: f64, t12190: f64, t12204: f64, t12240: f64, t17613: f64, t17616: f64, t17619: f64, t17623: f64, t17626: f64, t17627: f64, t17632: f64, t17638: f64, t17641: f64, t2265: f64, t631: f64, t8718: f64) -> f64 {
    let t17669 = t637 * t639 * t17667;
    let t17673 = -t2265 * t17613 / 3.0_f64 + 2.0_f64 / 27.0_f64 * t2265 * t17616 + 2.0_f64 / 9.0_f64 * t12143 * t17619 - 2.0_f64 / 3.0_f64 * t2265 * t17623 + t17626 + t8718 + t12132 - t17627 / 3.0_f64 + t12162 + t12164 + 10.0_f64 / 27.0_f64 * t12165 - 2.0_f64 / 9.0_f64 * t2265 * t17632 - 4.0_f64 / 9.0_f64 * t12171 + 2.0_f64 * t2265 * t17638 + 4.0_f64 / 3.0_f64 * t2265 * t17641 + t12174 - t12190 + t631 * t17669 / 2.0_f64 + 10.0_f64 / 9.0_f64 * t12204 - t12240;
    t17673
}
