//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1211/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1211<F: Float>(t127767: F, t7060: F, t786: F, t122037: F, t27341: F, t103452: F, t121991: F, t121992: F, t121993: F, t121998: F, t126345: F, t126358: F, t14587: F, t1949: F, t27206: F, t28425: F, t32426: F, t32463: F, t34054: F) -> F {
    let t127827 = t786 * t127767 * t7060;
    let t127833 = t122037 * t27341;
    let t127841 = F::cast_from(0.7437465841810202164e-3_f64) * t126345 + t121991 - t121992 + F::cast_from(0.25389723392137995738e-1_f64) * t121993 + t121998 + F::cast_from(0.14456046980341999104e-1_f64) * t127827 + F::cast_from(0.34271842599061411569e1_f64) * t32463 * t103452 * t1949 * t14587 + F::cast_from(0.51405703062096148813e-1_f64) * t127833 - F::cast_from(0.3718732920905101082e-3_f64) * t126358 + F::cast_from(0.57119737665102352616e0_f64) * t32426 * t34054 - F::cast_from(0.11423947533020470523e1_f64) * t32463 * t28425 * t27206;
    t127841
}
