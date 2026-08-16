//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1302/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1302(t2749: f64, t606: f64, t23285: f64, t2752: f64, t2745: f64, t10143: f64, t6665: f64, t2379: f64, t13487: f64, t10046: f64, t1880: f64, t214: f64, t225: f64, t258: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t81521 = t606 * t2749;
    let t81525 = t23285 * t2752;
    let t81529 = t606 * t2745;
    let t81539 = t6665 * t10143;
    let t81543 = t606 * t2379;
    let t81547 = t2752 * t606;
    let t81548 = t81547 * t13487;
    let t81554 = t1880 * t214 * t10046 * t225 * t258;
    (t81521, t81525, t81529, t81539, t81543, t81548, t81554)
}
