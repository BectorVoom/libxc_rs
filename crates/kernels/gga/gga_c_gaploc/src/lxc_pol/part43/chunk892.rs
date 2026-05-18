//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 892/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk892<F: Float>(t13086: F, t64: F, t10657: F, t871: F, t40612: F, t40614: F, t40620: F, t40630: F, t40632: F, t40634: F, t2558: F, t33360: F, t9647: F) -> (F, F, F, F, F, F, F, F, F) {
    let t43071 = F::new(4.0) / F::new(3.0) * t13086 * t64;
    let t43072 = t10657 * t871;
    let t43075 = F::new(7.0) / F::new(512.0) * t40612;
    let t43076 = F::new(63.0) / F::new(16384.0) * t40614;
    let t43077 = F::new(63.0) / F::new(1048576.0) * t40620;
    let t43078 = F::new(21.0) / F::new(1048576.0) * t40630;
    let t43079 = F::new(21.0) / F::new(16384.0) * t40632;
    let t43080 = F::new(7.0) / F::new(1536.0) * t40634;
    let t43093 = t9647 * t33360 * t2558;
    (t43071, t43072, t43075, t43076, t43077, t43078, t43079, t43080, t43093)
}
