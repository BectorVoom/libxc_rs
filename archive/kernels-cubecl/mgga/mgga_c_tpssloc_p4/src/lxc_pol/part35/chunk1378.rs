//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1378/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1378<F: Float>(t1527: F, t23270: F, t25038: F, t98224: F, t105519: F, t105567: F, t105604: F, t105650: F, t105689: F, t1912: F, t21054: F, t25348: F, t28317: F, t4268: F, t5658: F, t6627: F, t67305: F, t68322: F, t855: F, t858: F, t98993: F, t98995: F, t99003: F) -> F {
    let t105698 = t25038 * t23270 * t98224 * t1527;
    let t105700 = -F::cast_from(3.0_f64) * t67305 * t1912 - F::cast_from(0.34543615403812755166e0_f64) * t98993 - F::cast_from(0.57572692339687925277e-1_f64) * t98995 - t68322 * t1912 - F::cast_from(3.0_f64) * t25348 * t5658 + F::cast_from(0.57572692339687925277e-1_f64) * t99003 + F::cast_from(0.9869604401089358619e-1_f64) * t105519 + F::cast_from(6.0_f64) * t4268 * t28317 - t855 * t858 * (t105567 + t105604 + t105650 + t105689) + F::cast_from(6.0_f64) * t6627 * t21054 - F::cast_from(0.14804406601634037928e0_f64) * t105698;
    t105700
}
