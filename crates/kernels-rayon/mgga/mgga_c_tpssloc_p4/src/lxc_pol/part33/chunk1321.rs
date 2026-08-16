//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1321/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1321(t1888: f64, t23270: f64, t25044: f64, t5657: f64, t1880: f64, t25224: f64, t28263: f64, t105419: f64, t105423: f64, t105428: f64, t17092: f64, t21034: f64, t218: f64, t259: f64, t28432: f64, t4268: f64, t6627: f64, t7538: f64, t86955: f64, t86991: f64, t98237: f64) -> f64 {
    let t105437 = t1888 * t23270 * t25044 * t5657;
    let t105441 = t1880 * t25224 * t28263;
    let t105443 = -0.74022033008170189643e-1_f64 * t98237 + 0.19190897446562641759e0_f64 * t86955 + t218 * t105419 * t259 - 0.49348022005446793095e-1_f64 * t105423 + 0.82246703342411321825e-2_f64 * t105428 - 3.0_f64 * t4268 * t28432 - t6627 * t21034 - 6.0_f64 * t17092 * t7538 + 0.49348022005446793095e-1_f64 * t105437 - 0.19190897446562641759e0_f64 * t86991 - 0.24674011002723396548e-1_f64 * t105441;
    t105443
}
