//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 938/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk938<F: Float>(t872: F, t9062: F, t2347: F, t30005: F, t7990: F, t8419: F, t7987: F, t8423: F, t7306: F, t8397: F, t2331: F, t394: F) -> (F, F, F, F, F, F) {
    let t33459 = F::new(0.13170898365871023197e1) * t9062 * t872;
    let t33465 = t30005 * t2347;
    let t33468 = F::new(0.17347256376410398924e1) * t7990 * t8419;
    let t33475 = F::new(0.17347256376410398924e1) * t7987 * t8423;
    let t33488 = F::new(0.34694512752820797848e1) * t8397 * t7306;
    let t33489 = t394 * t2331;
    (t33459, t33465, t33468, t33475, t33488, t33489)
}
