//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 979/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk979(t133: f64, t138866: f64, t550: f64, t5551: f64, t3392: f64, t39: f64, t39801: f64, t40: f64, t136825: f64, t32774: f64, t32775: f64, t136898: f64, t5824: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t138870 = t133 * t138866;
    let t138873 = t550 * t5551;
    let t138874 = t133 * t138873;
    let t138879 = t3392 * t39801 * t39 * t40;
    let t138888 = t32774 * t136825 * t32775;
    let t138891 = t5824 * t136898;
    (t138870, t138873, t138874, t138879, t138888, t138891)
}
