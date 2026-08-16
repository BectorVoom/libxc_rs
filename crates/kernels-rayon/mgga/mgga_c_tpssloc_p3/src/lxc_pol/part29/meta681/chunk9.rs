//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2302/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2302(t24574: f64, t27474: f64, t27492: f64, t85853: f64, t27498: f64, t1215: f64, t1244: f64, t1246: f64, t15239: f64, t2144: f64, t24833: f64, t24858: f64, t27520: f64, t27721: f64, t3624: f64, t3625: f64, t4733: f64, t7283: f64, t7362: f64, t7373: f64, t8073: f64, t85920: f64, t85988: f64, t85996: f64, t86000: f64, t95109: f64) -> f64 {
    let t95125 = 0.18277045187202515961e-2_f64 * t24574 * t27474;
    let t95134 = 0.10966227112321509577e-1_f64 * t85853 * t27492;
    let t95136 = 0.54831135561607547884e-2_f64 * t85853 * t27498;
    let t95150 = -0.27415567780803773942e-2_f64 * t85988 - t95125 - 0.54831135561607547884e-2_f64 * t7283 * t7362 * t24858 * t4733 - t3624 * t95109 * t3625 + 0.18277045187202515961e-2_f64 * t85996 + t86000 + t95134 - t95136 - 0.82246703342411321825e-2_f64 * t7373 * t85920 * t8073 - 0.16449340668482264365e-1_f64 * t7373 * t24833 * t27520 + 2.0_f64 * t1244 * t27721 * t1215 * t1246 + t1244 * t2144 * t15239 * t1246;
    t95150
}
