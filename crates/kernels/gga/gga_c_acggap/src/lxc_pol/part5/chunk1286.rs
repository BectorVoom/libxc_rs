//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1286/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1286<F: Float>(t174: F, t5674: F, t5079: F, t944: F, t3409: F, t6400: F, t1298: F, t1841: F, t3476: F, t1131: F, t1165: F, t1531: F, t1532: F, t18436: F, t18458: F, t1899: F, t301: F, t335: F, t3462: F, t3463: F, t367: F, t372: F, t406: F, t4289: F, t5746: F, t5747: F, t5922: F, t6100: F, t6288: F, t839: F, t929: F, t960: F) -> (F, F) {
    let t23804 = t174 * t5674;
    let t23821 = t944 * t5079;
    let t23831 = t3409 * t6400;
    let t23838 = t944 * t1298;
    let t23849 = t3476 * t1841;
    let t23852 = t335 * t960 * t23804 * t301 / F::new(24.0) + t367 * t960 * t6100 * t372 / F::new(24.0) + t367 * t960 * t1899 * t1131 / F::new(48.0) + t335 * t960 * t6288 * t839 / F::new(48.0) + F::new(0.85748036236139473944e-3) * t1531 * t1165 * t1532 * t23821 + F::new(0.68598428988911579156e-2) * t3462 * t1165 * t5922 * t3463 * t372 + F::new(0.80031500487063509014e-2) * t23831 - F::new(0.34299214494455789578e-2) * t18436 - F::new(0.68598428988911579156e-2) * t3462 * t1165 * t4289 * t5747 - F::new(0.68598428988911579156e-2) * t3462 * t1165 * t1532 * t23838 * t406 - F::new(0.34299214494455789578e-2) * t3462 * t1165 * t1532 * t5746 * t929 - F::new(0.21437009059034868486e-3) * t23849 + F::new(0.16006300097412701803e-1) * t18458;
    (t23821, t23852)
}
