//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 748/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk748(t470: f64, t4835: f64, t472: f64, t542: f64, t1447: f64, t1218: f64, t156: f64, t1392: f64, t1396: f64, t4: f64, t427: f64, t1448: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4836 = t470 * t4835;
    let t4837 = 0.35089340384731224426e1_f64 * t4836;
    let t4838 = t542 * t472;
    let t4839 = t1447 * t4838;
    let t4840 = 0.21687161765563048428e-1_f64 * t4839;
    let t4841 = t156 * t1218;
    let t4842 = t1447 * t4841;
    let t4843 = 0.32530742648344572643e-1_f64 * t4842;
    let t4844 = t156 * t1392;
    let t4845 = t1447 * t4844;
    let t4846 = 0.48159446095139119799e0_f64 * t4845;
    let t4847 = t156 * t1396;
    let t4848 = t1447 * t4847;
    let t4849 = 0.16265371324172286321e-1_f64 * t4848;
    let t4850 = t427 * t4;
    let t4851 = t4850 * t1448;
    (t4837, t4840, t4843, t4846, t4849, t4851)
}
