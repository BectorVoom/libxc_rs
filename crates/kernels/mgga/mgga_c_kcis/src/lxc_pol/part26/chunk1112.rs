//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1112/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1112<F: Float>(t3245: F, t7928: F, t27543: F, t3733: F, t12520: F, t491: F, t12564: F, t4188: F, t7938: F, t12939: F, t2264: F, t4479: F, t7996: F, t12344: F, t2247: F, t1598: F, t251: F, t40512: F) -> (F, F, F, F, F, F, F, F, F) {
    let t94669 = t3245 * t7928;
    let t94748 = t3733 * t27543;
    let t94785 = t12520 * t491;
    let t94805 = t12564 * t491;
    let t94816 = t7938 * t4188;
    let t94819 = t2264 * t12939;
    let t94824 = t7996 * t4479;
    let t94833 = t2247 * t12344;
    let t94861 = t40512 * t251 * t1598;
    (t94669, t94748, t94785, t94805, t94816, t94819, t94824, t94833, t94861)
}
