//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1171/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1171<F: Float>(t10195: F, t10198: F, t10199: F, t10823: F, t10829: F, t10931: F, t10935: F, t113: F, t14408: F, t1934: F, t2904: F, t2956: F, t2962: F, t2963: F, t332: F, t43304: F, t43311: F, t44642: F, t44789: F, t5: F, t505: F, t8608: F, t889: F, t910: F, t911: F) -> F {
    let t44795 = t889 * t910 * t8608 + F::cast_from(3.0_f64) * t2904 * t10935 + t889 * t10931 * t505 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t889 * t43304 * t332 * t113 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t10829 * t2963 + t43311 * t911 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t889 * t2962 * t113 * t2956 + F::cast_from(3.0_f64) * t889 * t10198 * t14408 + F::cast_from(3.0_f64) * t2904 * t10199 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t889 * t2962 * t1934 + F::cast_from(3.0_f64) * t2904 * t10823 + t2904 * t10195 + t5 * (t44642 + t44789) * t332 * t113 / F::cast_from(4.0_f64);
    t44795
}
