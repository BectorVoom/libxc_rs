//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 710/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk710<F: Float>(t1423: F, t409: F, t1333: F, t461: F, t428: F, t1319: F, t456: F, t4607: F, t470: F, t472: F, t542: F, t1447: F, t1218: F, t156: F, t1392: F, t1396: F) -> (F, F, F, F, F, F, F, F) {
    let t4823 = t409 * t1423;
    let t4825 = t1333 * t461;
    let t4826 = 60.0 * t4825;
    let t4830 = t1333 * t428;
    let t4835 = t1319 * t4607 * t456;
    let t4836 = t470 * t4835;
    let t4837 = 0.35089340384731224426e1 * t4836;
    let t4838 = t542 * t472;
    let t4839 = t1447 * t4838;
    let t4840 = 0.21687161765563048428e-1 * t4839;
    let t4841 = t156 * t1218;
    let t4842 = t1447 * t4841;
    let t4843 = 0.32530742648344572643e-1 * t4842;
    let t4844 = t156 * t1392;
    let t4845 = t1447 * t4844;
    let t4846 = 0.48159446095139119799e0 * t4845;
    let t4847 = t156 * t1396;
    (t4823, t4826, t4830, t4837, t4840, t4843, t4846, t4847)
}
