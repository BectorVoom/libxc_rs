//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1941/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1941(t17030: f64, t232: f64, t6646: f64, t1888: f64, t16815: f64, t2632: f64, t22996: f64, t1909: f64, t226: f64, t23174: f64, t25310: f64, t26613: f64, t26667: f64, t26673: f64, t28407: f64, t28409: f64, t28411: f64, t28413: f64, t28420: f64, t5575: f64, t812: f64) -> (f64, f64, f64, f64, f64) {
    let t28422 = t17030 * t232;
    let t28423 = t6646 * t28422;
    let t28424 = t1888 * t28423;
    let t28426 = t16815 * t2632;
    let t28427 = t22996 * t28426;
    let t28428 = t1888 * t28427;
    let t28430 = t226 * t28407 - t23174 + t26613 - t812 * t28409 - t812 * t28411 + 2.0_f64 * t812 * t28413 - t26667 + t5575 * t1909 + 0.76763589786250567036e-1_f64 * t25310 + t26673 - 0.16449340668482264365e-1_f64 * t28420 - 0.82246703342411321825e-2_f64 * t28424 + 0.16449340668482264365e-1_f64 * t28428;
    (t28422, t28423, t28426, t28427, t28430)
}
