//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1186/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1186(t1955: f64, t3206: f64, t3174: f64, t10160: f64, t1052: f64, t1066: f64, t1920: f64, t1956: f64, t23346: f64, t23354: f64, t23359: f64, t23366: f64, t23369: f64, t23372: f64, t3169: f64, t3176: f64, t3207: f64, t6687: f64, t6695: f64, t6771: f64, t6816: f64) -> (f64, f64) {
    let t23377 = t1955 * t3206;
    let t23378 = t3174 * t23377;
    let t23381 = 0.43864908449286038306e-1_f64 * t23346 * t6695 + 0.82246703342411321825e-2_f64 * t1920 * t23354 - t23359 - 2.0_f64 * t3169 * t6816 - t6771 * t3207 + 2.0_f64 * t6771 * t3176 - 0.16449340668482264365e-1_f64 * t6687 * t23366 - 2.0_f64 * t23369 * t1066 - 2.0_f64 * t23372 * t1066 - 2.0_f64 * t10160 * t1956 + 2.0_f64 * t1052 * t23378;
    (t23378, t23381)
}
