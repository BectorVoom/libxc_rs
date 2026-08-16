//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 415/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk415(t79: f64, t1300: f64, t1603: f64, t1669: f64, t5538: f64, t5569: f64, t5598: f64, t5610: f64, t5611: f64, t6428: f64, t6431: f64, t6434: f64, t6438: f64, t6442: f64, t6446: f64, t6450: f64) -> f64 {
    let t80 = 0.1e-59_f64 < t79;
    let t6454 = piecewise3(t80, -0.23254900946437792e-1_f64 * t1603 * t6428 - 2.0_f64 * t1669 * t6431 + 0.25845121844514357744e-4_f64 * t5538 * t6434 + 0.22227677429409423704e-2_f64 * t1300 * t6438 + 0.22270151833971792333e-3_f64 * t5569 * t6442 + 0.38306165027777777778e-1_f64 * t5598 * t6446 - t5610 - 0.6384360837962962963e-2_f64 * t5611 * t6450, 0.0_f64);
    t6454
}
