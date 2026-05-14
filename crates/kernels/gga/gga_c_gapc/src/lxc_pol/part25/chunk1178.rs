//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1178/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1178<F: Float>(t34428: F, t34436: F, t34439: F, t34442: F, t34449: F, t34454: F, t34457: F, t34460: F, t34463: F, t34466: F, t34469: F, t34474: F, t34477: F, t34484: F, t34486: F, t34489: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36937 = 0.3243554543208642639e-2 * t34428;
    let t36939 = 0.15006749152217248259e-7 * t34436;
    let t36940 = 0.21720231316129303386e-4 * t34439;
    let t36941 = 0.2318836277704281739e-4 * t34442;
    let t36945 = 0.57920616843011475696e-5 * t34449;
    let t36946 = 0.50680539737635041234e-3 * t34454;
    let t36947 = 0.10298285674687440379e-4 * t34457;
    let t36948 = 0.10298285674687440379e-4 * t34460;
    let t36949 = 0.6070699179094394313e-6 * t34463;
    let t36950 = 0.14068827330203670243e-7 * t34466;
    let t36951 = 0.43284943850479925795e-3 * t34469;
    let t36952 = 0.80966145833333333338e-4 * t34474;
    let t36953 = 0.2845640240200497334e-7 * t34477;
    let t36956 = 0.10762101632577401621e-6 * t34484;
    let t36957 = 0.13259557375557346398e-6 * t34486;
    let t36958 = 0.4637672555408563478e-4 * t34489;
    (t36937, t36939, t36940, t36941, t36945, t36946, t36947, t36948, t36949, t36950, t36951, t36952, t36953, t36956, t36957, t36958)
}
