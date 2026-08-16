//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1391/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1391(t34466: f64, t34469: f64, t34474: f64, t34477: f64, t34484: f64, t34486: f64, t34489: f64, t34492: f64, t34495: f64, t34497: f64, t34499: f64, t34501: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36950 = 0.14068827330203670243e-7_f64 * t34466;
    let t36951 = 0.43284943850479925795e-3_f64 * t34469;
    let t36952 = 0.80966145833333333338e-4_f64 * t34474;
    let t36953 = 0.2845640240200497334e-7_f64 * t34477;
    let t36956 = 0.10762101632577401621e-6_f64 * t34484;
    let t36957 = 0.13259557375557346398e-6_f64 * t34486;
    let t36958 = 0.4637672555408563478e-4_f64 * t34489;
    let t36959 = 0.4637672555408563478e-4_f64 * t34492;
    let t36960 = 0.15716995342493974597e-7_f64 * t34495;
    let t36961 = 0.42206481990611010728e-7_f64 * t34497;
    let t36962 = 0.14068827330203670243e-7_f64 * t34499;
    let t36963 = 0.12817572129705434851e-5_f64 * t34501;
    (t36950, t36951, t36952, t36953, t36956, t36957, t36958, t36959, t36960, t36961, t36962, t36963)
}
