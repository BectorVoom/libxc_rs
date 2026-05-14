//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1074/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1074<F: Float>(t122037: F, t27341: F, t103452: F, t121991: F, t121992: F, t121993: F, t121998: F, t126345: F, t126358: F, t127827: F, t14587: F, t1949: F, t27206: F, t28425: F, t32426: F, t32463: F, t34054: F) -> (F,) {
    let t127833 = t122037 * t27341;
    let t127841 = 0.7437465841810202164e-3 * t126345 + t121991 - t121992 + 0.25389723392137995738e-1 * t121993 + t121998 + 0.14456046980341999104e-1 * t127827 + 0.34271842599061411569e1 * t32463 * t103452 * t1949 * t14587 + 0.51405703062096148813e-1 * t127833 - 0.3718732920905101082e-3 * t126358 + 0.57119737665102352616e0 * t32426 * t34054 - 0.11423947533020470523e1 * t32463 * t28425 * t27206;
    (t127841,)
}
