//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1606/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1606<F: Float>(t12261: F, t12297: F, t16706: F, t16876: F, t17115: F, t17117: F, t20268: F, t20274: F, t20276: F, t20278: F, t20280: F, t20322: F, t20338: F, t20341: F, t20344: F, t20347: F, t20350: F, t20353: F, t20357: F, t20359: F, t20362: F, t20380: F) -> F {
    let t20382 = F::cast_from(0.91983333333333333333e-1_f64) * t12261 - t17115 - t17117 - F::new(0.27595e-1) * t20268 + F::cast_from(0.26837777777777777779e0_f64) * t16706 + F::cast_from(0.18396666666666666667e0_f64) * t16876 + F::new(0.82785e-1) * t20274 + F::cast_from(0.18396666666666666667e-1_f64) * t20276 - F::new(0.11038e0) * t20278 - F::new(0.5519e-1) * t20280 + t20322 + F::new(0.258925e1) * t20338 + F::new(0.16557e0) * t20341 - F::new(0.5519e-1) * t20344 - F::new(0.16557e0) * t20347 + F::new(0.33114e0) * t20350 + F::new(0.49671e0) * t20353 + F::cast_from(0.13418888888888888889e0_f64) * t12297 + F::new(0.19419375e1) * t20357 - F::new(0.258925e1) * t20359 - F::new(0.1294625e1) * t20362 + t20380;
    t20382
}
