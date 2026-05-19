//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1180/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1180<F: Float>(t26854: F, t7687: F, t93157: F, t46978: F, t7692: F, t7690: F, t1250: F, t32896: F, t2173: F, t7710: F, t10463: F, t3489: F) -> (F, F, F, F, F, F, F) {
    let t93606 = t7687 * t26854;
    let t93628 = F::cast_from(0.73697530864197530862e-3_f64) * t93157;
    let t93661 = t46978 * t7692;
    let t93662 = t7690 * t93661;
    let t93737 = t32896 * t1250;
    let t93759 = t2173 * t46978 * t7710;
    let t93762 = t2173 * t93661;
    let t93779 = t10463 * t3489;
    (t93606, t93628, t93662, t93737, t93759, t93762, t93779)
}
