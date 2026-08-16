//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 723/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk723(t2021: f64, t3439: f64, t636: f64, t6778: f64, t6782: f64, t6787: f64, t6792: f64, t6795: f64, t6797: f64, t6800: f64, t6804: f64, t6870: f64, t6873: f64, t6876: f64, t6881: f64, t6885: f64, t6889: f64, t6894: f64, t6899: f64) -> f64 {
    let t6900 = -0.81498388966769604888e-2_f64 * t636 * t6778 + 0.32599355586707841954e-1_f64 * t636 * t6782 - 0.65198711173415683909e-1_f64 * t2021 * t6787 + 0.16299677793353920977e-1_f64 * t2021 * t6792 - 0.30426065214260652491e0_f64 * t6795 + 0.38032581517825815615e-1_f64 * t6797 - 0.7606516303565163123e-1_f64 * t6800 + 0.16299677793353920977e0_f64 * t3439 * t6804 - 0.27166129655589868296e-2_f64 * t636 * t6870 + 0.38032581517825815615e-1_f64 * t6873 - 0.16299677793353920978e-1_f64 * t6876 * t6881 + 0.16299677793353920977e-1_f64 * t2021 * t6885 - 0.27166129655589868296e-2_f64 * t636 * t6889 - 0.21551796193434628848e0_f64 * t6894 - t6899;
    t6900
}
