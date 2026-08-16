//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1192/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1192<F: Float>(t1934: F, t6722: F, t1933: F, t40: F, t1937: F, t3: F, t607: F, t343: F, t984: F) -> (F, F, F, F, F) {
    let t6723 = t6722 * t1934;
    let t6726 = t1933 * t40;
    let t6728 = F::cast_from(0.10093189023535097714e-3_f64) * t6726 * t1937;
    let t6729 = t3 * t607;
    let t6730 = t1933 * t6729;
    let t6733 = t984 * t343;
    (t6723, t6728, t6729, t6730, t6733)
}
