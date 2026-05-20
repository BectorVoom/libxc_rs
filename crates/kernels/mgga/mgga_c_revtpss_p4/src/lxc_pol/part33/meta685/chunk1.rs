//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2264/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2264<F: Float>(t112757: F, t7642: F, t104521: F, t105046: F, t105354: F, t105409: F, t105499: F, t111825: F, t111991: F, t1203: F, t1214: F, t1294: F, t1775: F, t1828: F, t21506: F, t2151: F, t26949: F, t26979: F, t29111: F, t29118: F, t29186: F, t30758: F, t30763: F, t30886: F, t30899: F, t5497: F, t6573: F, t6579: F, t7627: F, t7636: F, t7637: F, t7643: F, t7645: F, t7648: F, t7652: F, t8197: F, t8205: F, t97066: F, t97397: F) -> F {
    let t112880 = t7642 * t112757;
    let t112899 = -F::cast_from(0.4336814094102599731e0_f64) * t105354 * t111825 * t21506 - F::cast_from(0.4336814094102599731e0_f64) * t7648 * t30899 + F::cast_from(0.34694512752820797848e1_f64) * t7636 * t7652 * t29186 * t1828 + F::cast_from(0.10408353825846239354e2_f64) * t105046 * t2151 * t6579 * t1214 + F::cast_from(0.10408353825846239354e2_f64) * t105409 * t2151 * t6579 * t1294 - F::cast_from(0.17347256376410398924e1_f64) * t97397 * t30763 * t111991 - F::cast_from(0.34694512752820797848e1_f64) * t26979 * t30758 - F::cast_from(0.13170898365871023197e1_f64) * t104521 * t1775 - F::cast_from(0.26020884564615598386e1_f64) * t26949 * t7637 * t7627 * t6573 - F::cast_from(0.69389025505641595696e1_f64) * t97066 * t2151 * t6579 * t1203 + F::cast_from(0.8673628188205199462e0_f64) * t112880 * t7645 + F::cast_from(0.34694512752820797848e1_f64) * t7636 * t7652 * t8197 * t5497 - F::cast_from(0.13170898365871023197e1_f64) * t105499 * t1775 + F::cast_from(0.34694512752820797848e1_f64) * t7636 * t7652 * t30886 * t1203 - F::cast_from(0.34694512752820797848e1_f64) * t7643 * t7652 * t29118 * t1828 - F::cast_from(0.8673628188205199462e0_f64) * t8205 * t29111;
    t112899
}
