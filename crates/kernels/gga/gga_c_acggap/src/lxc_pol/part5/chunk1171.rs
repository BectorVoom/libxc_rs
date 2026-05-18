//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1171/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1171<F: Float>(t3431: F, t5722: F, t1165: F, t3194: F, t5284: F, t5852: F, t1173: F, t1181: F, t13031: F, t13040: F, t13065: F, t1531: F, t16328: F, t16332: F, t16356: F, t16359: F, t16373: F, t1899: F, t3196: F, t5136: F, t5862: F) -> F {
    let t21166 = t3431 * t5722;
    let t21170 = t3194 * t1165 * t5852 * t5284;
    let t21182 = -F::new(0.13719685797782315831e-1) * t16328 + F::new(0.20579528696673473746e-1) * t16332 - F::new(0.34299214494455789578e-2) * t1173 * t1165 * t1899 * t3196 - F::new(0.16006300097412701803e-1) * t21166 - F::new(0.17149607247227894789e-2) * t21170 - F::new(0.34299214494455789578e-2) * t13031 - F::new(0.42874018118069736972e-3) * t13040 - F::new(0.85748036236139473944e-3) * t16356 + F::new(455.0) / F::new(324.0) * t13065 + F::new(35.0) / F::new(108.0) * t16359 - F::new(0.51448821741683684366e-2) * t16373 + F::new(0.17149607247227894789e-2) * t1531 * t1181 * t5862 * t5136;
    t21182
}
