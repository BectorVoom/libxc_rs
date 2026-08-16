//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 774/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk774(t1034: f64, t8863: f64, t1040: f64, t3061: f64, t3065: f64, t3060: f64, t3072: f64, t3076: f64, t3138: f64, t3144: f64, t8830: f64, t8833: f64, t8835: f64, t8844: f64, t8849: f64, t8854: f64, t8856: f64, t8859: f64, t8861: f64) -> (f64, f64, f64, f64, f64) {
    let t8864 = t8863 * t1034;
    let t8865 = t8864 * t1040;
    let t8867 = t3061 * t3065;
    let t8869 = t3060 * t3072;
    let t8870 = t8869 * t3076;
    let t8872 = t3060 * t3138;
    let t8873 = t8872 * t3144;
    let t8875 = -0.6487109086417285278e-2_f64 * t8830 - 0.10120768229166666667e-3_f64 * t8833 + 0.1081184847736214213e-1_f64 * t8835 + 0.10005749997240850277e-7_f64 * t8844 + 0.84412963981222021454e-7_f64 * t8849 + 0.20011499994481700554e-7_f64 * t8854 + 0.19738380876484260726e-4_f64 * t8856 - 0.2318836277704281739e-4_f64 * t8859 - 0.10821235962619981449e-3_f64 * t8861 - 0.84412963981222021454e-7_f64 * t8865 - 0.16882592796244404291e-6_f64 * t8867 - 0.10005749997240850277e-7_f64 * t8870 - 0.49240895655712845848e-7_f64 * t8873;
    (t8865, t8867, t8870, t8873, t8875)
}
