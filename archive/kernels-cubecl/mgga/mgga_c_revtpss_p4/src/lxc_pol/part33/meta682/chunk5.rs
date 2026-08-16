//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2238/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2238<F: Float>(t1770: F, t8190: F, t104549: F, t105220: F, t1294: F, t1295: F, t20714: F, t21082: F, t21390: F, t2142: F, t26937: F, t26976: F, t27025: F, t29129: F, t29136: F, t29183: F, t29233: F, t29247: F, t30735: F, t30744: F, t30763: F, t30764: F, t30767: F, t30768: F, t30870: F, t30907: F, t7637: F, t7643: F, t7651: F, t7652: F, t7666: F, t8213: F, t96929: F, t97348: F, t97377: F, t97422: F) -> F {
    let t112075 = t1770 * t8190;
    let t112092 = -F::cast_from(0.17347256376410398924e1_f64) * t27025 * t30744 - F::cast_from(0.26020884564615598386e1_f64) * t26937 * t30768 + F::cast_from(0.8673628188205199462e0_f64) * t7643 * t7637 * t2142 * t21082 - F::cast_from(0.52041769129231196772e1_f64) * t97348 * t30763 * t96929 + F::cast_from(0.17347256376410398924e1_f64) * t29136 * t29233 - F::cast_from(0.8673628188205199462e0_f64) * t105220 * t8213 - F::cast_from(0.8673628188205199462e0_f64) * t29129 * t29247 + F::cast_from(0.10408353825846239354e2_f64) * t7651 * t97377 * t30767 * t1294 + F::cast_from(0.34694512752820797848e1_f64) * t27025 * t30907 - F::cast_from(0.13170898365871023197e1_f64) * t112075 * t1295 - F::cast_from(0.4336814094102599731e0_f64) * t30870 * t7666 + F::cast_from(0.17347256376410398924e1_f64) * t97422 * t30764 - F::cast_from(0.13170898365871023197e1_f64) * t26976 * t20714 - F::cast_from(0.26341796731742046394e1_f64) * t104549 * t21390 - F::cast_from(0.17347256376410398924e1_f64) * t7643 * t7652 * t30735 * t1294 + F::cast_from(0.13170898365871023197e1_f64) * t1770 * t29183;
    t112092
}
