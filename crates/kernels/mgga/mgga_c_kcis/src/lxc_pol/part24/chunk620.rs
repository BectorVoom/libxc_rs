//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 620/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk620<F: Float>(t6548: F, t6633: F, t393: F, t1820: F, t5036: F) -> (F, F, F, F) {
    let t6634 = t6548 + t6633;
    let t6635 = t6634 * t393;
    let t6637 = F::cast_from(2.0_f64) * t5036 * t1820;
    let t6638 = t1820 * t1820;
    (t6634, t6635, t6637, t6638)
}
