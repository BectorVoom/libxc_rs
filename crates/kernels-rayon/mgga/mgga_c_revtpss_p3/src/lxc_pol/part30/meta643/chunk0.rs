//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2249/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2249(t12627: f64, t1276: f64, t7635: f64, t1203: f64, t1774: f64, t1248: f64, t1770: f64, t7627: f64, t104606: f64, t1214: f64, t1287: f64, t1295: f64, t2151: f64, t26895: f64, t26969: f64, t27029: f64, t29118: f64, t29136: f64, t29163: f64, t29195: f64, t29213: f64, t29278: f64, t29287: f64, t29304: f64, t3585: f64, t3588: f64, t3738: f64, t3769: f64, t5231: f64, t5464: f64, t7643: f64, t8201: f64, t8209: f64, t96866: f64, t96874: f64, t96979: f64, t96986: f64, t97082: f64, t97343: f64) -> f64 {
    let t105269 = t12627 * t7635 * t1276;
    let t105270 = t1774 * t1203;
    let t105277 = t1248 * t1203;
    let t105284 = t1770 * t7627;
    let t105310 = 0.52041769129231196772e1_f64 * t7643 * t26969 * t8201 * t3738 + 0.17347256376410398924e1_f64 * t97082 * t29213 - 0.10408353825846239354e2_f64 * t105269 * t2151 * t105270 * t1214 + 0.8673628188205199462e0_f64 * t96874 * t8209 - 0.34694512752820797848e1_f64 * t96979 * t29195 * t5464 * t105277 + 0.26341796731742046394e1_f64 * t96866 * t5231 - 0.13170898365871023197e1_f64 * t105284 * t1295 - 0.65854491829355115987e0_f64 * t29304 * t3585 + 0.17347256376410398924e1_f64 * t96986 * t104606 * t3769 + 0.17347256376410398924e1_f64 * t26895 * t29118 * t1248 * t1287 + 0.8673628188205199462e0_f64 * t26895 * t8201 * t3588 * t1287 + 0.17347256376410398924e1_f64 * t97082 * t29163 + 0.17347256376410398924e1_f64 * t26895 * t29278 * t1248 * t1287 + 0.17347256376410398924e1_f64 * t29136 * t27029 + 0.34694512752820797848e1_f64 * t97343 * t29287;
    t105310
}
