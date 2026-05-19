//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 723/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk723<F: Float>(t2021: F, t3439: F, t636: F, t6778: F, t6782: F, t6787: F, t6792: F, t6795: F, t6797: F, t6800: F, t6804: F, t6870: F, t6873: F, t6876: F, t6881: F, t6885: F, t6889: F, t6894: F, t6899: F) -> F {
    let t6900 = -F::cast_from(0.81498388966769604888e-2_f64) * t636 * t6778 + F::cast_from(0.32599355586707841954e-1_f64) * t636 * t6782 - F::cast_from(0.65198711173415683909e-1_f64) * t2021 * t6787 + F::cast_from(0.16299677793353920977e-1_f64) * t2021 * t6792 - F::cast_from(0.30426065214260652491e0_f64) * t6795 + F::cast_from(0.38032581517825815615e-1_f64) * t6797 - F::cast_from(0.7606516303565163123e-1_f64) * t6800 + F::cast_from(0.16299677793353920977e0_f64) * t3439 * t6804 - F::cast_from(0.27166129655589868296e-2_f64) * t636 * t6870 + F::cast_from(0.38032581517825815615e-1_f64) * t6873 - F::cast_from(0.16299677793353920978e-1_f64) * t6876 * t6881 + F::cast_from(0.16299677793353920977e-1_f64) * t2021 * t6885 - F::cast_from(0.27166129655589868296e-2_f64) * t636 * t6889 - F::cast_from(0.21551796193434628848e0_f64) * t6894 - t6899;
    t6900
}
