//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 909/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk909<F: Float>(t40610: F, t2751: F, t10108: F, t257: F, t111: F, t3931: F, t2363: F, t576: F, t1395: F, t671: F, t1372: F, t794: F) -> (F, F, F, F, F, F, F) {
    let t40611 = F::cast_from(1.0_f64) / t40610;
    let t40771 = t2751 * t2751;
    let t40772 = F::cast_from(1.0_f64) / t40771;
    let t40889 = F::cast_from(1.0_f64) / t10108 / t257;
    let t45560 = t3931 * t111;
    let t55571 = t576 * t2363;
    let t66940 = t1395 * t671;
    let t80645 = t794 * t1372;
    (t40611, t40772, t40889, t45560, t55571, t66940, t80645)
}
