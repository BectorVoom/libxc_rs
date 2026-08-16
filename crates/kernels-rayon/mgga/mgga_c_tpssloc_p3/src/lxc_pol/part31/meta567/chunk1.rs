//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1799/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1799(t849: f64, t87340: f64, t1516: f64, t81763: f64, t23083: f64, t25094: f64, t23046: f64, t4184: f64, t812: f64, t836: f64, t242: f64, t81816: f64) -> (f64, f64, f64, f64, f64) {
    let t87341 = t87340 * t849;
    let t87345 = t81763 * t1516;
    let t87347 = t23083 * t25094;
    let t87363 = t812 * t23046 * t836 * t4184;
    let t87368 = t812 * t81816 * t242;
    (t87341, t87345, t87347, t87363, t87368)
}
