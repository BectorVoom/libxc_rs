//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1313/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1313<F: Float>(t12345: F, t1555: F, t28576: F, t4189: F, t4310: F, t8207: F, t2069: F, t94197: F, t4479: F, t8236: F, t28558: F, t1505: F, t28556: F) -> (F, F, F, F, F, F) {
    let t99713 = F::new(12.0) * t12345 * t28576 * t1555;
    let t99716 = F::new(2.0) * t4189 * t8207 * t4310;
    let t99717 = t94197 * t2069;
    let t99718 = t8236 * t4479;
    let t99723 = t28558 * t4310;
    let t99724 = t28556 * t1505;
    (t99713, t99716, t99717, t99718, t99723, t99724)
}
