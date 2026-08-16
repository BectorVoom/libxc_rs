//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 845/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk845<F: Float>(t6898: F, t1885: F, t2132: F, t446: F, t1650: F, t2011: F, t4171: F, t4170: F, t4160: F, t1889: F, t5632: F, t1395: F) -> (F, F, F, F, F, F) {
    let t6899 = t6898 / F::cast_from(16.0_f64);
    let t6900 = t1885 * t2132;
    let t6901 = t446 * t6900;
    let t6902 = t6901 / F::cast_from(8.0_f64);
    let t6903 = t1650 * t2011;
    let t6904 = t4171 * t6903;
    let t6905 = t4170 * t6904;
    let t6906 = t4160 * t6905;
    let t6908 = t5632 * t1889;
    let t6909 = t1395 * t6908;
    (t6899, t6902, t6905, t6906, t6908, t6909)
}
