//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2224/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2224<F: Float>(t355: F, t5352: F, t17288: F, t2142: F, t5216: F, t11239: F, t1811: F, t1276: F, t2148: F, t1209: F, t2143: F, t1203: F, t1215: F, t1295: F, t1770: F, t17988: F, t18019: F, t26886: F, t26909: F, t26994: F, t27008: F, t27011: F, t27020: F, t29212: F, t29236: F, t29278: F, t29296: F, t5215: F, t5237: F, t5245: F, t5497: F, t5498: F, t7602: F, t7627: F, t7636: F, t7637: F, t7643: F, t7651: F, t7652: F, t96927: F, t96953: F, t96954: F, t97041: F) -> (F, F, F) {
    let t104510 = t355 * t5352;
    let t104521 = t17288 * t2142;
    let t104524 = t5216 * t2142;
    let t104527 = t1811 * t11239;
    let t104529 = t2148 * t104527 * t1276;
    let t104549 = t1209 * t2143;
    let t104560 = -F::cast_from(0.52041769129231196772e1_f64) * t97041 * t29212 * t104510 + F::cast_from(0.13170898365871023197e1_f64) * t7602 * t18019 - F::cast_from(0.34694512752820797848e1_f64) * t96927 * t29236 * t96954 + F::cast_from(0.13170898365871023197e1_f64) * t27011 * t5237 - F::cast_from(0.13170898365871023197e1_f64) * t104521 * t1215 - F::cast_from(0.13170898365871023197e1_f64) * t104524 * t1295 - F::cast_from(0.8673628188205199462e0_f64) * t104529 * t26909 + F::cast_from(0.65854491829355115987e0_f64) * t1770 * t26886 - F::cast_from(0.13170898365871023197e1_f64) * t27008 * t5498 + F::cast_from(0.17347256376410398924e1_f64) * t7651 * t7652 * t7627 * t5497 + F::cast_from(0.34694512752820797848e1_f64) * t96953 * t29296 * t96954 - F::cast_from(0.13170898365871023197e1_f64) * t27020 * t5498 + F::cast_from(0.34694512752820797848e1_f64) * t26994 * t7637 * t29278 * t1203 - F::cast_from(0.26341796731742046394e1_f64) * t104549 * t17988 + F::cast_from(0.17347256376410398924e1_f64) * t7643 * t7637 * t7627 * t5245 - F::cast_from(0.17347256376410398924e1_f64) * t7636 * t7637 * t7627 * t5215;
    (t104510, t104529, t104560)
}
