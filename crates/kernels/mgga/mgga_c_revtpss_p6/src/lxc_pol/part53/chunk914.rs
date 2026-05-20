//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 914/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk914<F: Float>(t1097: F, t25591: F, t25605: F, t25640: F, t25651: F, t27595: F, t27599: F, t27606: F, t27609: F, t27616: F, t27621: F, t27627: F, t27631: F, t27635: F, t27640: F, t27643: F, t27647: F, t4758: F, t7144: F, t7156: F, t7162: F, t7167: F, t7170: F, t7174: F, t7825: F, t7833: F, t7837: F) -> F {
    let t27650 = F::cast_from(0.17347256376410398924e1_f64) * t7144 * t27595 + F::cast_from(0.17347256376410398924e1_f64) * t25591 * t27599 - F::cast_from(0.4336814094102599731e0_f64) * t7825 * t7174 - F::cast_from(0.4336814094102599731e0_f64) * t7167 * t27606 + F::cast_from(0.8673628188205199462e0_f64) * t27609 * t7162 - F::cast_from(0.4336814094102599731e0_f64) * t7156 * t7837 + F::cast_from(0.13170898365871023197e1_f64) * t25651 * t4758 - F::cast_from(0.65854491829355115987e0_f64) * t27616 * t1097 - F::cast_from(0.4336814094102599731e0_f64) * t27621 * t7170 - F::cast_from(0.4336814094102599731e0_f64) * t25640 * t7833 - F::cast_from(0.4336814094102599731e0_f64) * t7167 * t27627 - F::cast_from(0.4336814094102599731e0_f64) * t7167 * t27631 + F::cast_from(0.17347256376410398924e1_f64) * t7144 * t27635 + F::cast_from(0.4336814094102599731e0_f64) * t27640 * t27643 + F::cast_from(0.8673628188205199462e0_f64) * t25605 * t27647;
    t27650
}
