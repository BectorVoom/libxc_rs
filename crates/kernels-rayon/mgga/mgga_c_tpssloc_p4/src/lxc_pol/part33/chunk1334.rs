//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1334/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1334(t1527: f64, t23270: f64, t25038: f64, t98224: f64, t105519: f64, t105567: f64, t105604: f64, t105650: f64, t105689: f64, t1912: f64, t21054: f64, t25348: f64, t28317: f64, t4268: f64, t5658: f64, t6627: f64, t67305: f64, t68322: f64, t855: f64, t858: f64, t98993: f64, t98995: f64, t99003: f64) -> f64 {
    let t105698 = t25038 * t23270 * t98224 * t1527;
    let t105700 = -3.0_f64 * t67305 * t1912 - 0.34543615403812755166e0_f64 * t98993 - 0.57572692339687925277e-1_f64 * t98995 - t68322 * t1912 - 3.0_f64 * t25348 * t5658 + 0.57572692339687925277e-1_f64 * t99003 + 0.9869604401089358619e-1_f64 * t105519 + 6.0_f64 * t4268 * t28317 - t855 * t858 * (t105567 + t105604 + t105650 + t105689) + 6.0_f64 * t6627 * t21054 - 0.14804406601634037928e0_f64 * t105698;
    t105700
}
