//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2339/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2339<F: Float>(t10186: F, t10259: F, t13831: F, t13832: F, t13835: F, t13836: F, t13839: F, t13851: F, t13934: F, t2776: F, t2780: F, t2960: F, t2986: F, t2988: F, t42762: F, t42773: F, t42785: F, t42788: F, t42794: F, t42846: F, t43043: F, t43069: F, t4518: F, t4531: F, t47887: F, t47907: F, t47915: F, t47919: F, t47927: F, t47938: F, t6733: F) -> F {
    let t47940 = -F::cast_from(0.49999999999999999999e-2_f64) * t2986 * t2988 * t47887 + F::cast_from(0.16666666666666666666e-2_f64) * t2986 * t10259 * t13835 - F::cast_from(0.83333333333333333331e-3_f64) * t2986 * t4531 * t6733 * t2780 - F::cast_from(0.16666666666666666666e-2_f64) * t2986 * t13851 * t13831 + F::cast_from(0.16666666666666666666e-2_f64) * t2986 * t4531 * t43043 + F::cast_from(0.44444444444444444443e-2_f64) * t10186 * t13832 - F::cast_from(0.55555555555555555554e-3_f64) * t47907 - F::cast_from(0.12674897119341563785e-1_f64) * t42762 - F::cast_from(0.27777777777777777777e-3_f64) * t42773 - F::cast_from(0.27777777777777777777e-3_f64) * t42785 - F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t42846 * t13839 + F::cast_from(0.49999999999999999998e-2_f64) * t2986 * t4518 * t47915 + F::cast_from(0.16666666666666666666e-2_f64) * t2986 * t2988 * t47919 - F::cast_from(0.55555555555555555554e-3_f64) * t42788 + F::cast_from(0.55555555555555555554e-3_f64) * t42794 + F::cast_from(0.66666666666666666666e-2_f64) * t2960 * t13934 - F::cast_from(0.25925925925925925925e-2_f64) * t2986 * t43069 * t47927 + F::cast_from(0.16666666666666666666e-2_f64) * t2986 * t4531 * t6733 * t2776 - F::cast_from(0.88888888888888888885e-2_f64) * t10186 * t13836 + F::cast_from(0.11111111111111111111e-2_f64) * t47938;
    t47940
}
