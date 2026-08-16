//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1221/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1221(t105531: f64, t105543: f64, t105547: f64, t105551: f64, t105561: f64, t105565: f64, t20857: f64, t28997: f64, t29010: f64, t4166: f64, t812: f64, t84953: f64, t87068: f64, t87080: f64, t98330: f64, t98342: f64, t98345: f64, t98356: f64, t98363: f64, t98374: f64, t98380: f64) -> f64 {
    let t108164 = -0.3289868133696452873e-1_f64 * t105531 - 0.69087230807625510332e0_f64 * t98330 - 0.49348022005446793095e-1_f64 * t105543 - 0.29608813203268075857e0_f64 * t105547 - 0.19739208802178717238e0_f64 * t105551 - 0.15626873635058151147e0_f64 * t87068 - 0.24674011002723396548e-1_f64 * t98342 + 0.9869604401089358619e-1_f64 * t98345 + 0.49348022005446793095e-1_f64 * t98356 + 0.16449340668482264365e-1_f64 * t105561 - 6.0_f64 * t812 * t84953 * t20857 - 6.0_f64 * t4166 * t28997 - 0.49348022005446793095e-1_f64 * t105565 - 0.49348022005446793095e-1_f64 * t98363 - 0.11514538467937585055e0_f64 * t98374 + 0.38381794893125283518e0_f64 * t87080 + 0.11514538467937585055e0_f64 * t98380 - 3.0_f64 * t4166 * t29010;
    t108164
}
