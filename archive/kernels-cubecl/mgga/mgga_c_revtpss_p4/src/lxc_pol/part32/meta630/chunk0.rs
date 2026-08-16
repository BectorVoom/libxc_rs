//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2031/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2031<F: Float>(t30391: F, t689: F, t93314: F, t93302: F, t2718: F, t7997: F, t103212: F, t103521: F, t103529: F, t103543: F, t103547: F, t106275: F, t14587: F, t1580: F, t25383: F, t26550: F, t27353: F, t28400: F, t28425: F, t30357: F, t62604: F, t62637: F, t7420: F, t7766: F, t95945: F, t95948: F) -> F {
    let t110676 = t30391 * t689;
    let t110677 = t93314 * t110676;
    let t110679 = t93302 * t110676;
    let t110687 = t2718 * t7997;
    let t110694 = F::cast_from(0.4336814094102599731e0_f64) * t106275 * t7420 + F::cast_from(0.8673628188205199462e0_f64) * t25383 * t30357 - F::cast_from(0.8673628188205199462e0_f64) * t7766 * t28400 + F::cast_from(0.38549458614245330944e-1_f64) * t103521 - F::cast_from(0.13170898365871023197e1_f64) * t103212 * t1580 + F::cast_from(0.17135234354032049604e-2_f64) * t95945 - t103529 + F::cast_from(0.14456046980341999104e-1_f64) * t110677 - F::cast_from(0.25702851531048074406e-1_f64) * t110679 - F::cast_from(0.8673628188205199462e0_f64) * t27353 * t28425 * t62604 + F::cast_from(0.11565819519348392139e-2_f64) * t95948 + F::cast_from(0.86736281882051994624e-1_f64) * t103543 - F::cast_from(0.19274729307122665472e-1_f64) * t103547 - F::cast_from(0.17347256376410398924e1_f64) * t27353 * t110687 * t14587 + F::cast_from(0.4336814094102599731e0_f64) * t27353 * t26550 * t62637;
    t110694
}
