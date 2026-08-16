//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1327/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1327(t105531: f64, t105543: f64, t105547: f64, t105551: f64, t105561: f64, t105565: f64, t20853: f64, t25255: f64, t28351: f64, t28409: f64, t28413: f64, t4166: f64, t5575: f64, t5585: f64, t5617: f64, t6657: f64, t7535: f64, t812: f64, t87068: f64, t87142: f64, t98330: f64, t98342: f64, t98345: f64, t98356: f64) -> f64 {
    let t105567 = 6.0_f64 * t4166 * t28413 - 3.0_f64 * t4166 * t28409 + 3.0_f64 * t5575 * t7535 - 0.16449340668482264365e-1_f64 * t105531 - 3.0_f64 * t812 * t25255 * t5617 - 6.0_f64 * t4166 * t28351 + 6.0_f64 * t812 * t87142 * t5585 - 0.34543615403812755166e0_f64 * t98330 - 0.24674011002723396548e-1_f64 * t105543 - 0.14804406601634037928e0_f64 * t105547 - 0.9869604401089358619e-1_f64 * t105551 - 0.78134368175290755733e-1_f64 * t87068 - 0.12337005501361698274e-1_f64 * t98342 - t812 * t6657 * t20853 + 0.49348022005446793095e-1_f64 * t98345 + 0.24674011002723396548e-1_f64 * t98356 + 0.82246703342411321825e-2_f64 * t105561 - 0.24674011002723396548e-1_f64 * t105565;
    t105567
}
