//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2274/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2274<F: Float>(t1139: F, t24312: F, t1132: F, t1723: F, t6442: F, t12327: F, t12331: F, t12349: F, t12352: F, t24238: F, t24242: F, t24246: F, t24250: F, t24289: F, t24292: F, t24295: F, t24298: F) -> (F, F, F, F, F, F) {
    let t24313 = t1139 * t24312;
    let t24315 = t1132 * t24312;
    let t24317 = t6442 * t1723;
    let t24318 = t12327 * t24317;
    let t24320 = t12331 * t24317;
    let t24322 = F::new(0.17938e1) * t24242 + F::cast_from(0.29896666666666666667e0_f64) * t24250 - F::cast_from(0.16431333333333333333e0_f64) * t24289 + F::cast_from(0.49293999999999999999e0_f64) * t24292 + F::cast_from(0.82156666666666666667e-1_f64) * t24295 - t12349 - t12352 - F::cast_from(0.82156666666666666668e-1_f64) * t24298 - F::cast_from(0.59793333333333333333e0_f64) * t24238 + F::new(0.17938e1) * t24246 + F::new(0.3071625e0) * t24313 + F::new(0.1898925e1) * t24315 + F::cast_from(0.142419375e1_f64) * t24318 - F::new(0.76790625e-1) * t24320;
    (t24313, t24315, t24317, t24318, t24320, t24322)
}
