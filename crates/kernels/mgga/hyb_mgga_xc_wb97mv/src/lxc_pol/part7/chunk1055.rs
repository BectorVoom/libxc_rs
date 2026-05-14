//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1055/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1055<F: Float>(t10927: F, t2199: F, t4192: F, t808: F, t6919: F, t4189: F, t2247: F, t4188: F, t2245: F, t3369: F, t3373: F, t4162: F, t6862: F, t6859: F, t4229: F, t6965: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10929 = 4.0 * t2199 * t10927;
    let t10930 = t4192 * t808;
    let t10932 = 0.96491876992155210402e2 * t6919 * t10930;
    let t10933 = t4189 * t808;
    let t10935 = 2.0 * t2199 * t10933;
    let t10936 = t4188 * t2247;
    let t10937 = t10936 * t808;
    let t10939 = 0.16081979498692535067e2 * t2245 * t10937;
    let t10940 = t3373 * t3369;
    let t10942 = 0.32163958997385070134e2 * t2245 * t10940;
    let t10943 = t4162 * t6862;
    let t10944 = t10943 * t808;
    let t10946 = 0.51726012919273400301e3 * t6859 * t10944;
    let t10947 = t6965 * t4229;
    (t10929, t10930, t10932, t10933, t10935, t10936, t10937, t10939, t10940, t10942, t10943, t10944, t10946, t10947)
}
