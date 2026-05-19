//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 980/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk980<F: Float>(t1314: F, t3897: F, t455: F, t3900: F, t468: F, t11407: F, t1346: F, t3943: F, t3946: F, t481: F, t1311: F, t3860: F) -> (F, F, F, F, F, F) {
    let t11512 = F::new(1.0) / t3897 / t1314;
    let t11513 = t455 * t11512;
    let t11516 = F::new(1.0) / t3900 / t468;
    let t11520 = F::cast_from(0.28842592592592592592e-1_f64) * t11407;
    let t11536 = F::new(1.0) / t3943 / t1346;
    let t11539 = F::new(1.0) / t3946 / t481;
    let t11543 = t1311 * t3860;
    (t11513, t11516, t11520, t11536, t11539, t11543)
}
