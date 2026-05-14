//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1085/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1085<F: Float>(t5351: F, t8347: F, t2146: F, t2147: F, t33256: F, t33258: F, t33262: F, t38377: F, t38379: F, t38382: F, t38386: F, t38389: F, t38392: F, t38393: F, t5331: F, t633: F, t8108: F, t8126: F, t9003: F) -> (F,) {
    let t38397 = t8347 * t5351;
    let t38406 = -0.26020884564615598386e1 * t9003 * t8108 + t38377 - t38379 - t38382 - t38386 + t38389 - t38392 + 0.65854491829355115987e0 * t38393 - 0.17347256376410398924e1 * t33256 + 0.13170898365871023197e1 * t33258 - 0.13170898365871023197e1 * t38397 + 0.8673628188205199462e0 * t33262 + 0.8673628188205199462e0 * t2146 * t2147 * t633 * t5331 + 0.8673628188205199462e0 * t9003 * t8126;
    (t38406,)
}
