//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1247/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1247<F: Float>(t2138: F, t2147: F, t322: F, t9417: F, t1265: F, t2146: F, t33175: F, t33320: F, t33321: F, t33324: F, t38471: F, t38474: F, t38481: F, t38487: F, t38489: F, t556: F, t7912: F, t7931: F, t8301: F, t9025: F, t9136: F, t9391: F, t9414: F, t9418: F) -> F {
    let t38493 = F::new(0.34694512752820797848e1) * t2138 * t2147 * t9417 * t322;
    let t38503 = -t38471 - F::new(0.8673628188205199462e0) * t38474 + F::new(0.17347256376410398924e1) * t7912 * t9418 + F::new(0.17347256376410398924e1) * t7912 * t9414 + t38481 + F::new(0.8673628188205199462e0) * t2146 * t2147 * t8301 * t556 - t38487 + t38489 - t38493 - t33320 + F::new(0.17347256376410398924e1) * t33321 + F::new(0.8673628188205199462e0) * t7912 * t9136 - F::new(0.13170898365871023197e1) * t33324 - F::new(0.17347256376410398924e1) * t7931 * t33175 * t9025 - F::new(0.65854491829355115987e0) * t9391 * t1265;
    t38503
}
