//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1278/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1278<F: Float>(t384: F, t398: F, t429: F, t6192: F, t1089: F, t1193: F, t14113: F, t14115: F, t14117: F, t14120: F, t14122: F, t18295: F, t18297: F, t18299: F, t18301: F, t418: F, t422: F, t5679: F, t5876: F) -> F {
    let t23636 = t384 * t398 * t429 * t6192;
    let t23650 = -F::new(0.85748036236139473944e-3) * t14113 + F::new(0.42874018118069736972e-3) * t14115 - F::new(0.42874018118069736972e-3) * t14117 + t14120 + t14122 - F::new(0.85748036236139473944e-3) * t23636 - F::new(0.34299214494455789578e-2) * t418 * t1089 * t429 * t5876 - F::new(0.17149607247227894789e-2) * t418 * t422 * t5679 * t1193 + F::new(0.16006300097412701803e-1) * t18295 - F::new(0.16006300097412701803e-1) * t18297 + F::new(0.80031500487063509016e-2) * t18299 + F::new(0.34299214494455789578e-2) * t18301;
    t23650
}
