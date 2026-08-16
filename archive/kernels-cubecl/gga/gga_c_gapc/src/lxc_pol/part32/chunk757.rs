//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 757/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk757<F: Float>(t1034: F, t8863: F, t1040: F, t3061: F, t3065: F, t3060: F, t3072: F, t3076: F, t3138: F, t3144: F, t8830: F, t8833: F, t8835: F, t8844: F, t8849: F, t8854: F, t8856: F, t8859: F, t8861: F) -> F {
    let t8864 = t8863 * t1034;
    let t8865 = t8864 * t1040;
    let t8867 = t3061 * t3065;
    let t8869 = t3060 * t3072;
    let t8870 = t8869 * t3076;
    let t8872 = t3060 * t3138;
    let t8873 = t8872 * t3144;
    let t8875 = -F::cast_from(0.6487109086417285278e-2_f64) * t8830 - F::cast_from(0.10120768229166666667e-3_f64) * t8833 + F::cast_from(0.1081184847736214213e-1_f64) * t8835 + F::cast_from(0.10005749997240850277e-7_f64) * t8844 + F::cast_from(0.84412963981222021454e-7_f64) * t8849 + F::cast_from(0.20011499994481700554e-7_f64) * t8854 + F::cast_from(0.19738380876484260726e-4_f64) * t8856 - F::cast_from(0.2318836277704281739e-4_f64) * t8859 - F::cast_from(0.10821235962619981449e-3_f64) * t8861 - F::cast_from(0.84412963981222021454e-7_f64) * t8865 - F::cast_from(0.16882592796244404291e-6_f64) * t8867 - F::cast_from(0.10005749997240850277e-7_f64) * t8870 - F::cast_from(0.49240895655712845848e-7_f64) * t8873;
    t8875
}
