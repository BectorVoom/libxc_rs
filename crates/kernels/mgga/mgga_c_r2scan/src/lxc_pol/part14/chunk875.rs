//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 875/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk875<F: Float>(t2155: F, t7949: F, t551: F, t552: F, t7591: F, t5109: F, t7356: F, t2207: F, t2208: F, t2837: F, t2612: F, t495: F) -> (F, F, F, F, F) {
    let t7951 = F::new(0.19514881078765566037e-1) * t2155 * t7949;
    let t7953 = t551 * t552 * t7591;
    let t7956 = t5109 * t7356;
    let t7961 = t2207 * t2837 * t2208;
    let t7963 = t2612 * t495;
    (t7951, t7953, t7956, t7961, t7963)
}
