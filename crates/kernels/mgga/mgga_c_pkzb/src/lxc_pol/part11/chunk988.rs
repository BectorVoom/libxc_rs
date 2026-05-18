//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 988/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk988<F: Float>(t1066: F, t3515: F, t218: F, t219: F, t10767: F, t208: F, t10769: F, t10801: F, t10803: F, t10807: F, t10812: F, t10814: F, t10816: F, t5543: F, t5558: F, t7332: F, t7357: F, t9148: F, t9185: F, t9192: F) -> (F, F, F, F, F) {
    let t10821 = t1066 * t3515;
    let t10823 = t218 * t219 * t10821;
    let t10825 = t208 * t10767;
    let t10827 = t218 * t219 * t10825;
    let t10829 = F::new(0.19419375e1) * t10801 - F::new(0.3883875e1) * t10803 + F::new(0.258925e1) * t10807 - t5543 + F::new(0.12077e1) * t7357 - F::new(0.905775e0) * t9148 + F::new(0.905775e0) * t10769 - F::new(0.412621875e-1) * t10812 + F::new(0.247573125e0) * t10814 + F::new(0.16504875e0) * t10816 - t5558 + F::new(0.82785e0) * t7332 - F::new(0.49671e0) * t9185 - F::new(0.49671e0) * t9192 + F::new(0.745065e0) * t10823 + F::new(0.248355e0) * t10827;
    (t10821, t10823, t10825, t10827, t10829)
}
