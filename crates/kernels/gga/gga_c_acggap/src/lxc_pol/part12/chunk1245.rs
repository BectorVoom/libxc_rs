//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1245/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1245<F: Float>(t157: F, t2146: F, t2152: F, t2338: F, t2395: F, t29994: F, t33293: F, t33294: F, t33301: F, t33306: F, t33308: F, t33675: F, t38209: F, t38383: F, t38441: F, t38443: F, t38453: F, t38455: F, t38458: F, t7912: F, t7931: F, t8303: F, t9386: F) -> F {
    let t38469 = -t33293 + t38441 + F::new(0.8673628188205199462e0) * t38443 + F::new(0.65854491829355115987e0) * t33294 + F::new(0.34694512752820797848e1) * t7931 * t38383 * t33675 + F::new(0.8673628188205199462e0) * t29994 * t2395 + F::new(0.17347256376410398924e1) * t38453 - t33301 + F::new(0.34694512752820797848e1) * t38455 + t38458 - F::new(0.4336814094102599731e0) * t2338 * t8303 + F::new(0.4336814094102599731e0) * t2146 * t2152 * t38209 * t157 - F::new(0.34694512752820797848e1) * t33306 + F::new(0.8673628188205199462e0) * t7912 * t9386 + F::new(0.13170898365871023197e1) * t33308;
    t38469
}
