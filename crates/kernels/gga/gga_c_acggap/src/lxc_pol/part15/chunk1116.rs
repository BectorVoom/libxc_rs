//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1116/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1116<F: Float>(t32003: F, t38052: F, t8406: F, t1915: F, t8331: F, t2222: F, t2236: F, t33256: F, t33266: F, t33271: F, t38386: F, t38389: F, t38392: F, t38393: F, t38397: F, t38415: F, t38418: F, t40620: F, t6425: F) -> (F,) {
    let t42194 = t32003 * t38052 * t8406;
    let t42200 = t8331 * t1915;
    let t42205 = -t38386 + t38389 - t38392 + 0.13170898365871023197e1 * t38393 + 0.34694512752820797848e1 * t42194 - 0.8673628188205199462e0 * t33256 + 0.26341796731742046394e1 * t2222 * t6425 - 0.26341796731742046394e1 * t38397 + 0.13170898365871023197e1 * t42200 + 0.8673628188205199462e0 * t40620 * t2236 - t38415 + t38418 - 0.8673628188205199462e0 * t33266 - t33271;
    (t42205,)
}
