//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1217/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1217<F: Float>(t1101: F, t1181: F, t1894: F, t3361: F, t1165: F, t21118: F, t6138: F, t3409: F, t5869: F, t1090: F, t17085: F, t17088: F, t17090: F, t17092: F, t17105: F, t17107: F, t17109: F, t1899: F, t3396: F, t3403: F, t4919: F, t5862: F) -> F {
    let t22220 = t3361 * t1181 * t1894 * t1101;
    let t22236 = t3361 * t1165 * t6138 * t21118;
    let t22238 = t3409 * t5869;
    let t22243 = F::new(0.34299214494455789578e-2) * t22220 + F::new(0.68598428988911579156e-2) * t3396 * t1181 * t1899 * t1090 - F::new(0.42874018118069736972e-2) * t3403 * t1165 * t5862 * t4919 + F::new(0.85748036236139473944e-3) * t17085 + F::new(0.12004725073059526352e-1) * t17088 + F::new(0.68598428988911579156e-2) * t17090 + F::new(0.85748036236139473945e-2) * t17092 + F::new(0.20579528696673473746e-1) * t22236 - F::new(0.40015750243531754508e-2) * t22238 - F::new(0.45351183609335988443e-1) * t17105 + F::new(0.45351183609335988443e-1) * t17107 - F::new(0.22675591804667994222e-1) * t17109;
    t22243
}
