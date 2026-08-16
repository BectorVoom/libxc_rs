//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1396/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1396(t34454: f64, t34457: f64, t34460: f64, t34463: f64, t34466: f64, t34469: f64, t34474: f64, t34477: f64, t34484: f64, t34486: f64, t34489: f64, t34492: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36946 = 0.50680539737635041234e-3_f64 * t34454;
    let t36947 = 0.10298285674687440379e-4_f64 * t34457;
    let t36948 = 0.10298285674687440379e-4_f64 * t34460;
    let t36949 = 0.6070699179094394313e-6_f64 * t34463;
    let t36950 = 0.14068827330203670243e-7_f64 * t34466;
    let t36951 = 0.43284943850479925795e-3_f64 * t34469;
    let t36952 = 0.80966145833333333338e-4_f64 * t34474;
    let t36953 = 0.2845640240200497334e-7_f64 * t34477;
    let t36956 = 0.10762101632577401621e-6_f64 * t34484;
    let t36957 = 0.13259557375557346398e-6_f64 * t34486;
    let t36958 = 0.4637672555408563478e-4_f64 * t34489;
    let t36959 = 0.4637672555408563478e-4_f64 * t34492;
    (t36946, t36947, t36948, t36949, t36950, t36951, t36952, t36953, t36956, t36957, t36958, t36959)
}
