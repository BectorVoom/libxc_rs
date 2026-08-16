//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 368/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk368(t6187: f64, t762: f64, t242: f64, t1901: f64, t193: f64, t446: f64, t6073: f64, t6076: f64, t6081: f64, t6085: f64, t6090: f64, t6094: f64, t6099: f64, t6101: f64, t6105: f64, t6150: f64, t6156: f64, t6160: f64, t6163: f64, t6168: f64, t6172: f64, t6177: f64, t89: f64) -> (f64, f64) {
    let t6188 = t762 * t6187;
    let t6189 = t242 * t6188;
    let t6192 = t6073 + t1901 * t6076 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t6081 - t446 * t6085 / 3.0_f64 + t446 * t6090 / 3.0_f64 - t446 * t6094 / 3.0_f64 - t6099 - t446 * t6101 / 9.0_f64 - t446 * t6105 / 3.0_f64 + t89 * t193 * t6150 / 3.0_f64 - t446 * t6156 / 3.0_f64 + t6160 + t1901 * t6163 / 9.0_f64 + t446 * t6168 / 3.0_f64 - t446 * t6172 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t6177 - t446 * t6189 / 3.0_f64;
    (t6189, t6192)
}
