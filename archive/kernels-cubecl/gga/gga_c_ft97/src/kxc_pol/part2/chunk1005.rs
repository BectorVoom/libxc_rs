//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 1005/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk1005<F: Float>(t15549: F, t332: F, t113: F, t1259: F, t14569: F, t14571: F, t14576: F, t14579: F, t14582: F, t14593: F, t1577: F, t1934: F, t2900: F, t2958: F, t2966: F, t333: F, t4318: F, t4322: F, t5: F, t505: F, t889: F, t911: F, t992: F) -> F {
    let t15550 = t15549 * t332;
    let t15554 = -t889 * t14569 + t14571 * t911 / F::cast_from(2.0_f64) + t4322 * t2966 / F::cast_from(2.0_f64) + t889 * t14576 / F::cast_from(2.0_f64) + t889 * t14579 / F::cast_from(4.0_f64) + t889 * t14582 / F::cast_from(4.0_f64) + t5 * t4318 * t505 / F::cast_from(2.0_f64) + t5 * t2900 * t992 / F::cast_from(4.0_f64) + t4322 * t2958 / F::cast_from(4.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t889 * t14593 - t5 * t333 * t1577 / F::cast_from(2.0_f64) + t5 * t1259 * t1934 / F::cast_from(4.0_f64) + t5 * t15550 * t113 / F::cast_from(4.0_f64);
    t15554
}
