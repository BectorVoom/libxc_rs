//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 737/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk737<F: Float>(t10869: F, t10929: F, t332: F, t113: F, t2962: F, t505: F, t10195: F, t10199: F, t10819: F, t10823: F, t10829: F, t10832: F, t1934: F, t2900: F, t2904: F, t2958: F, t2963: F, t2966: F, t333: F, t5: F, t8608: F, t886: F, t889: F, t911: F) -> (F, F, F, F, F) {
    let t10930 = t10869 + t10929;
    let t10931 = t10930 * t332;
    let t10932 = t10931 * t113;
    let t10935 = t2962 * t505;
    let t10943 = 3.0 / 4.0 * t2904 * t2958 + 3.0 / 2.0 * t2904 * t2966 + t889 * t10195 / 4.0 + 3.0 / 4.0 * t889 * t10199 + t5 * t333 * t8608 / 4.0 + t5 * t10819 * t113 / 4.0 + 3.0 / 4.0 * t889 * t10823 + 3.0 / 4.0 * t5 * t886 * t1934 + 3.0 / 4.0 * t10829 * t911 + 3.0 / 4.0 * t889 * t10832 + t889 * t10932 / 4.0 + 3.0 / 4.0 * t889 * t10935 + 3.0 / 4.0 * t2904 * t2963 + 3.0 / 4.0 * t5 * t2900 * t505;
    (t10930, t10931, t10932, t10935, t10943)
}
