//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 925/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk925<F: Float>(t1097: F, t25591: F, t25605: F, t25640: F, t25651: F, t27595: F, t27599: F, t27606: F, t27609: F, t27616: F, t27621: F, t27627: F, t27631: F, t27635: F, t27640: F, t27643: F, t27647: F, t4758: F, t7144: F, t7156: F, t7162: F, t7167: F, t7170: F, t7174: F, t7825: F, t7833: F, t7837: F) -> F {
    let t27650 = F::new(0.17347256376410398924e1) * t7144 * t27595 + F::new(0.17347256376410398924e1) * t25591 * t27599 - F::new(0.4336814094102599731e0) * t7825 * t7174 - F::new(0.4336814094102599731e0) * t7167 * t27606 + F::new(0.8673628188205199462e0) * t27609 * t7162 - F::new(0.4336814094102599731e0) * t7156 * t7837 + F::new(0.13170898365871023197e1) * t25651 * t4758 - F::new(0.65854491829355115987e0) * t27616 * t1097 - F::new(0.4336814094102599731e0) * t27621 * t7170 - F::new(0.4336814094102599731e0) * t25640 * t7833 - F::new(0.4336814094102599731e0) * t7167 * t27627 - F::new(0.4336814094102599731e0) * t7167 * t27631 + F::new(0.17347256376410398924e1) * t7144 * t27635 + F::new(0.4336814094102599731e0) * t27640 * t27643 + F::new(0.8673628188205199462e0) * t25605 * t27647;
    t27650
}
