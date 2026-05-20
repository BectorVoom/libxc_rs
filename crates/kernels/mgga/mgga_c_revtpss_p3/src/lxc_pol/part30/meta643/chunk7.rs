//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2256/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2256<F: Float>(t12640: F, t7635: F, t1248: F, t1294: F, t18043: F, t18109: F, t1828: F, t26884: F, t26901: F, t26937: F, t26949: F, t26979: F, t27015: F, t27020: F, t27025: F, t29129: F, t29136: F, t29148: F, t29175: F, t29195: F, t29237: F, t29268: F, t29272: F, t29293: F, t3568: F, t5429: F, t5464: F, t7602: F, t7632: F, t7637: F, t7651: F, t7652: F, t8201: F, t8208: F, t8209: F, t96954: F, t96966: F, t97313: F, t97348: F, t97358: F) -> F {
    let t105644 = t12640 * t7635;
    let t105657 = F::cast_from(0.34694512752820797848e1_f64) * t97313 * t29195 * t5464 * t1248 * t1294 + F::cast_from(0.26341796731742046394e1_f64) * t27020 * t5429 - F::cast_from(0.34694512752820797848e1_f64) * t26979 * t29237 + F::cast_from(0.26341796731742046394e1_f64) * t7632 * t18109 + F::cast_from(0.13170898365871023197e1_f64) * t7602 * t18043 + F::cast_from(0.52041769129231196772e1_f64) * t26949 * t7652 * t8208 * t3568 - F::cast_from(0.52041769129231196772e1_f64) * t97348 * t29148 * t96954 - F::cast_from(0.4336814094102599731e0_f64) * t29129 * t26901 + F::cast_from(0.10408353825846239354e2_f64) * t97358 * t7637 * t8201 * t3568 + F::cast_from(0.17347256376410398924e1_f64) * t96966 * t8209 - F::cast_from(0.34694512752820797848e1_f64) * t29136 * t27015 + F::cast_from(0.34694512752820797848e1_f64) * t105644 * t29268 + F::cast_from(0.8673628188205199462e0_f64) * t7651 * t7652 * t26884 * t1828 - F::cast_from(0.34694512752820797848e1_f64) * t26979 * t29293 - F::cast_from(0.17347256376410398924e1_f64) * t27025 * t29175 + F::cast_from(0.17347256376410398924e1_f64) * t26937 * t29272;
    t105657
}
