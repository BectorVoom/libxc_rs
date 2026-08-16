//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1396/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1396(t101774: f64, t103957: f64, t29664: f64, t29667: f64, t29670: f64, t29672: f64, t29674: f64, t91785: f64, t91786: f64, t97626: f64, t99790: f64, t99791: f64) -> f64 {
    let tv4rho3sigma8 = t101774 - t91785 - t97626 + t91786 - t29664 - t29667 - t29670 + t29672 - t99790 + t29674 - t99791 + t103957;
    tv4rho3sigma8
}
