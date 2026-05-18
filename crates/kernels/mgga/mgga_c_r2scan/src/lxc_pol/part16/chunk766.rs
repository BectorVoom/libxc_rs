//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 766/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk766<F: Float>(t1527: F, t2788: F, t4983: F, t2461: F, t879: F, t2321: F, t955: F, t6897: F, t986: F, t5021: F, t5872: F, t5874: F) -> (F, F, F, F, F, F, F, F) {
    let t6959 = t2788 * t1527;
    let t6961 = F::new(48.0) * t4983;
    let t6963 = F::new(2.0) * t879 * t2461;
    let t6966 = t2321 * t955;
    let t6967 = t986 * t6897;
    let t7025 = F::new(4.0) * t5021;
    let t7026 = F::new(1584.0) * t5872;
    let t7027 = F::new(1872.0) * t5874;
    (t6959, t6961, t6963, t6966, t6967, t7025, t7026, t7027)
}
