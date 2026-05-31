//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1109/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1109<F: Float>(t2961: F, t10194: F, t10819: F, t10829: F, t10832: F, t10930: F, t10932: F, t113: F, t1934: F, t2900: F, t2904: F, t2957: F, t2958: F, t2966: F, t332: F, t333: F, t39370: F, t43088: F, t43140: F, t43183: F, t43270: F, t4381: F, t5: F, t505: F, t8608: F, t886: F, t889: F, t909: F) -> F {
    let t43290 = t2961 * t2961;
    let t43297 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t5 * t2900 * t1934 + t5 * t10819 * t505 + F::cast_from(3.0_f64) * t2904 * t10832 + F::cast_from(3.0_f64) * t10829 * t2966 + t2904 * t10932 + t889 * (t43088 + t43140 + t43183 + t43270) * t332 * t113 / F::cast_from(4.0_f64) + t5 * t886 * t8608 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t889 * t2957 * t1934 + t889 * t10930 * t909 * t4381 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t10829 * t2958 + t5 * t333 * t39370 / F::cast_from(4.0_f64) + t889 * t43290 * t332 * t113 / F::cast_from(4.0_f64) + t889 * t10194 * t505;
    t43297
}
