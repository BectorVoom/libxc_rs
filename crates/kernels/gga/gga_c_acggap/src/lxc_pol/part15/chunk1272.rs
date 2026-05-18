//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1272/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1272<F: Float>(t33796: F, t9168: F, t2131: F, t2132: F, t309: F, t9971: F, t10017: F, t2138: F, t2147: F, t322: F, t157: F, t1915: F, t2146: F, t2152: F, t2338: F, t33301: F, t38455: F, t38458: F, t38471: F, t38474: F, t38481: F, t38662: F, t406: F, t463: F, t8004: F, t8316: F, t9422: F, t9428: F) -> F {
    let t42229 = t33796 * t9168;
    let t42247 = t2131 * t2132 * t9971 * t309;
    let t42252 = t2138 * t2147 * t10017 * t322;
    let t42256 = t2138 * t2132 * t9971 * t322;
    let t42258 = -F::new(0.17347256376410398924e1) * t38662 * t9428 + F::new(0.17347256376410398924e1) * t42229 - t33301 + F::new(0.69389025505641595696e1) * t38455 + t38458 - F::new(0.8673628188205199462e0) * t2338 * t9422 - F::new(0.26020884564615598386e1) * t2146 * t8004 * t10017 * t463 + F::new(0.4336814094102599731e0) * t2146 * t2152 * t9971 * t406 * t157 + F::new(0.13170898365871023197e1) * t8316 * t1915 - F::new(0.8673628188205199462e0) * t42247 - t38471 - F::new(0.17347256376410398924e1) * t38474 - F::new(0.17347256376410398924e1) * t42252 + t38481 + F::new(0.8673628188205199462e0) * t42256;
    t42258
}
