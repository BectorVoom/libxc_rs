//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2249/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2249<F: Float>(t12627: F, t1276: F, t7635: F, t1203: F, t1774: F, t1248: F, t1770: F, t7627: F, t104606: F, t1214: F, t1287: F, t1295: F, t2151: F, t26895: F, t26969: F, t27029: F, t29118: F, t29136: F, t29163: F, t29195: F, t29213: F, t29278: F, t29287: F, t29304: F, t3585: F, t3588: F, t3738: F, t3769: F, t5231: F, t5464: F, t7643: F, t8201: F, t8209: F, t96866: F, t96874: F, t96979: F, t96986: F, t97082: F, t97343: F) -> F {
    let t105269 = t12627 * t7635 * t1276;
    let t105270 = t1774 * t1203;
    let t105277 = t1248 * t1203;
    let t105284 = t1770 * t7627;
    let t105310 = F::cast_from(0.52041769129231196772e1_f64) * t7643 * t26969 * t8201 * t3738 + F::cast_from(0.17347256376410398924e1_f64) * t97082 * t29213 - F::cast_from(0.10408353825846239354e2_f64) * t105269 * t2151 * t105270 * t1214 + F::cast_from(0.8673628188205199462e0_f64) * t96874 * t8209 - F::cast_from(0.34694512752820797848e1_f64) * t96979 * t29195 * t5464 * t105277 + F::cast_from(0.26341796731742046394e1_f64) * t96866 * t5231 - F::cast_from(0.13170898365871023197e1_f64) * t105284 * t1295 - F::cast_from(0.65854491829355115987e0_f64) * t29304 * t3585 + F::cast_from(0.17347256376410398924e1_f64) * t96986 * t104606 * t3769 + F::cast_from(0.17347256376410398924e1_f64) * t26895 * t29118 * t1248 * t1287 + F::cast_from(0.8673628188205199462e0_f64) * t26895 * t8201 * t3588 * t1287 + F::cast_from(0.17347256376410398924e1_f64) * t97082 * t29163 + F::cast_from(0.17347256376410398924e1_f64) * t26895 * t29278 * t1248 * t1287 + F::cast_from(0.17347256376410398924e1_f64) * t29136 * t27029 + F::cast_from(0.34694512752820797848e1_f64) * t97343 * t29287;
    t105310
}
