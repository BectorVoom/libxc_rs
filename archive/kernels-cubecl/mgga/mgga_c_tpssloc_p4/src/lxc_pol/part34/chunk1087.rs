//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1087/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1087<F: Float>(t40610: F, t2751: F, t10108: F, t257: F, t68: F, t1406: F, t9238: F, t111: F, t6470: F, t2239: F, t5385: F, t20292: F) -> (F, F, F, F, F, F, F) {
    let t40611 = F::cast_from(1.0_f64) / t40610;
    let t40771 = t2751 * t2751;
    let t40772 = F::cast_from(1.0_f64) / t40771;
    let t40889 = F::cast_from(1.0_f64) / t10108 / t257;
    let t40890 = t68 * t40889;
    let t45844 = t1406 * t9238;
    let t55388 = t6470 * t111;
    let t55921 = t5385 * t2239;
    let t67001 = t20292 * t111;
    (t40611, t40772, t40890, t45844, t55388, t55921, t67001)
}
