//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 753/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk753<F: Float>(t1109: F, t7203: F, t7479: F, t33384: F, t6758: F, t6798: F, t7853: F, t238: F, t27566: F, t27717: F, t33372: F, t33383: F, t33418: F, t33443: F, t35402: F, t35446: F, t35449: F, t35453: F, t35457: F, t35460: F, t35462: F, t3789: F, t6829: F, t7453: F, t7477: F) -> (F, F, F) {
    let t35466 = t7203 * t1109;
    let t35467 = t35466 * t7479;
    let t35470 = t33384 * t6758;
    let t35473 = t7853 * t6798;
    let t35480 = -0.26350381008313446725e-3 * t238 * t35402 + 0.10338048737805743097e-3 * t27566 * t35446 + 0.15322466011111111111e0 * t33372 * t35449 - 0.18164417702296932716e-2 * t35453 * t35457 + t33443 + 0.1443087735596363459e-7 * t3789 * t35460 * t35462 - 0.31303728621554032609e-1 * t7477 * t35467 + 0.25845121844514357744e-4 * t33418 * t35470 + 0.88910709717637694816e-2 * t27717 * t35473 - 0.25845121844514357744e-4 * t33383 * t35470 + 0.20429954681481481482e0 * t7453 * t6829;
    (t35466, t35467, t35480)
}
