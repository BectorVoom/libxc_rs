//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1392/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1392<F: Float>(t34421: F, t34424: F, t34426: F, t34428: F, t34436: F, t34439: F, t34442: F, t34449: F, t34454: F, t34457: F, t34460: F, t34463: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36934 = F::new(0.48917046440972222224e-4) * t34421;
    let t36935 = F::new(0.25002399603899953676e-2) * t34424;
    let t36936 = F::new(0.3243554543208642639e-2) * t34426;
    let t36937 = F::new(0.3243554543208642639e-2) * t34428;
    let t36939 = F::new(0.15006749152217248259e-7) * t34436;
    let t36940 = F::new(0.21720231316129303386e-4) * t34439;
    let t36941 = F::new(0.2318836277704281739e-4) * t34442;
    let t36945 = F::new(0.57920616843011475696e-5) * t34449;
    let t36946 = F::new(0.50680539737635041234e-3) * t34454;
    let t36947 = F::new(0.10298285674687440379e-4) * t34457;
    let t36948 = F::new(0.10298285674687440379e-4) * t34460;
    let t36949 = F::new(0.6070699179094394313e-6) * t34463;
    (t36934, t36935, t36936, t36937, t36939, t36940, t36941, t36945, t36946, t36947, t36948, t36949)
}
