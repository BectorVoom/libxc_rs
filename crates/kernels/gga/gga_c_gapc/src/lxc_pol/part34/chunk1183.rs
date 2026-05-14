//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1183/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1183<F: Float>(t34421: F, t34424: F, t34426: F, t34428: F, t34436: F, t34439: F, t34442: F, t34410: F, t34413: F, t34417: F, t34433: F, t34449: F, t34454: F, t34457: F, t34460: F, t34463: F) -> (F, F, F, F, F, F) {
    let t36934 = 0.48917046440972222224e-4 * t34421;
    let t36935 = 0.25002399603899953676e-2 * t34424;
    let t36936 = 0.3243554543208642639e-2 * t34426;
    let t36937 = 0.3243554543208642639e-2 * t34428;
    let t36939 = 0.15006749152217248259e-7 * t34436;
    let t36940 = 0.21720231316129303386e-4 * t34439;
    let t36941 = 0.2318836277704281739e-4 * t34442;
    let t36942 = 0.66297786877786731988e-7 * t34410 + 0.98332751566569010434e-8 * t34413 - 0.89048050908546122981e-5 * t34417 - t36934 + t36935 + t36936 + t36937 - 0.44198524585191154658e-7 * t34433 - t36939 + t36940 - t36941;
    let t36945 = 0.57920616843011475696e-5 * t34449;
    let t36946 = 0.50680539737635041234e-3 * t34454;
    let t36947 = 0.10298285674687440379e-4 * t34457;
    let t36948 = 0.10298285674687440379e-4 * t34460;
    let t36949 = 0.6070699179094394313e-6 * t34463;
    (t36942, t36945, t36946, t36947, t36948, t36949)
}
