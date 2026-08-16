//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1956/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1956<F: Float>(t1395: F, t5456: F, t2105: F, t6470: F, t1851: F, t7961: F, t1404: F, t1858: F, t20149: F, t20186: F, t2099: F, t27241: F, t29396: F, t5364: F, t5381: F, t6483: F, t7223: F, t7946: F, t91830: F, t91832: F, t91834: F, t91842: F) -> (F, F) {
    let t100930 = t1395 * t5456;
    let t100966 = t6470 * t2105;
    let t100972 = t1851 * t7961;
    let t100976 = t1404 * t29396 + F::cast_from(2.0_f64) * t1858 * t27241 + t20149 * t2105 + t20186 * t2099 + F::cast_from(2.0_f64) * t5364 * t7961 + F::cast_from(2.0_f64) * t5381 * t7946 + t6483 * t7223 + t100966 + F::cast_from(2.0_f64) * t100972 + t91830 + t91832 + t91834 + t91842;
    (t100930, t100976)
}
