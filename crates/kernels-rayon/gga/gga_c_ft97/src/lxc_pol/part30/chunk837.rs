//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 837/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk837(t1109: f64, t7203: f64, t7479: f64, t33384: f64, t6758: f64, t6798: f64, t7853: f64, t238: f64, t27566: f64, t27717: f64, t33372: f64, t33383: f64, t33418: f64, t33443: f64, t35402: f64, t35446: f64, t35449: f64, t35453: f64, t35457: f64, t35460: f64, t35462: f64, t3789: f64, t6829: f64, t7453: f64, t7477: f64) -> (f64, f64, f64) {
    let t35466 = t7203 * t1109;
    let t35467 = t35466 * t7479;
    let t35470 = t33384 * t6758;
    let t35473 = t7853 * t6798;
    let t35480 = -0.26350381008313446725e-3_f64 * t238 * t35402 + 0.10338048737805743097e-3_f64 * t27566 * t35446 + 0.15322466011111111111e0_f64 * t33372 * t35449 - 0.18164417702296932716e-2_f64 * t35453 * t35457 + t33443 + 0.1443087735596363459e-7_f64 * t3789 * t35460 * t35462 - 0.31303728621554032609e-1_f64 * t7477 * t35467 + 0.25845121844514357744e-4_f64 * t33418 * t35470 + 0.88910709717637694816e-2_f64 * t27717 * t35473 - 0.25845121844514357744e-4_f64 * t33383 * t35470 + 0.20429954681481481482e0_f64 * t7453 * t6829;
    (t35466, t35467, t35480)
}
