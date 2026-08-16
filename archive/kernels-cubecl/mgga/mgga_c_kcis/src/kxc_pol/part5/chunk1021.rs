//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1021/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1021<F: Float>(t25: F, t4973: F, t285: F, t4958: F, t984: F, t4943: F, t9938: F, t991: F, t2880: F, t291: F, t4581: F, t9959: F) -> (F, F, F, F, F) {
    let t14425 = t25 * t4973;
    let t14427 = t285 * t14425 / F::cast_from(144.0_f64);
    let t14439 = t984 * t4958 / F::cast_from(54.0_f64);
    let t14440 = t9938 * t4943;
    let t14442 = t991 * t14440 / F::cast_from(432.0_f64);
    let t14443 = t2880 * t291;
    let t14444 = t14443 * t4581;
    let t14446 = t991 * t14444 / F::cast_from(216.0_f64);
    let t14447 = t9959 * t291;
    (t14427, t14439, t14442, t14446, t14447)
}
