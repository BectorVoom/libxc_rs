//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1349/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1349<F: Float>(t3075: F, t5004: F, t359: F, t4930: F, t999: F, t1043: F, t1089: F, t4757: F, t3291: F, t4772: F, t1678: F, t3133: F) -> (F, F, F, F, F) {
    let t16446 = t5004 * t3075;
    let t16449 = t359 * t4930;
    let t16450 = t16449 * t999;
    let t16458 = t4757 * t1043 * t1089;
    let t16461 = t3291 * t4772;
    let t16465 = t1678 * t3133 * t1089;
    (t16446, t16450, t16458, t16461, t16465)
}
