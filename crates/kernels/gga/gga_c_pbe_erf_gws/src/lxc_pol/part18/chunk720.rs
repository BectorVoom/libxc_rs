//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 720/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk720<F: Float>(t1447: F, t4838: F, t1218: F, t156: F, t1392: F, t1396: F, t4: F, t427: F, t1448: F, t1438: F, t461: F, t4358: F, t88: F, t4560: F, t1332: F, t408: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4839 = t1447 * t4838;
    let t4840 = 0.21687161765563048428e-1 * t4839;
    let t4841 = t156 * t1218;
    let t4842 = t1447 * t4841;
    let t4843 = 0.32530742648344572643e-1 * t4842;
    let t4844 = t156 * t1392;
    let t4845 = t1447 * t4844;
    let t4846 = 0.48159446095139119799e0 * t4845;
    let t4847 = t156 * t1396;
    let t4848 = t1447 * t4847;
    let t4849 = 0.16265371324172286321e-1 * t4848;
    let t4850 = t427 * t4;
    let t4851 = t4850 * t1448;
    let t4853 = t1438 * t461;
    let t4855 = t4358 * t88;
    let t4856 = 24.0 * t4855;
    let t4857 = t4560 * t88;
    let t4859 = t408 * t1332;
    (t4840, t4843, t4846, t4849, t4851, t4853, t4856, t4857, t4859)
}
