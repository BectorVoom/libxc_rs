//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1016/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1016<F: Float>(t3010: F, t320: F, t315: F, t11132: F, t11337: F, t963: F, t3013: F, t323: F, t2873: F, t910: F, t2942: F, t941: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11465 = F::new(1.0) / t3010 / t320;
    let t11466 = t315 * t11465;
    let t11479 = F::new(0.93932222222222222223e0) * t11132;
    let t11480 = F::new(0.36793333333333333333e0) * t11337;
    let t11506 = F::new(1.0) / t3010 / t963;
    let t11507 = t315 * t11506;
    let t11509 = F::new(1.0) / t3013 / t323;
    let t11528 = t910 * t2873;
    let t11534 = F::new(0.55403703703703703703e-1) * t11132;
    let t11548 = t941 * t2942;
    (t11465, t11466, t11479, t11480, t11506, t11507, t11509, t11528, t11534, t11548)
}
