//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1415/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1415(t34410: f64, t34413: f64, t34417: f64, t34433: f64, t36934: f64, t36935: f64, t36936: f64, t36937: f64, t36939: f64, t36940: f64, t36941: f64, t34505: f64, t34510: f64, t36956: f64, t36957: f64, t36958: f64, t36959: f64, t36960: f64, t36961: f64, t36962: f64, t36963: f64, t36965: f64) -> (f64, f64) {
    let t38571 = 0.6629778687778673199e-7_f64 * t34410 + 0.98332751566569010432e-8_f64 * t34413 - 0.89048050908546122982e-5_f64 * t34417 - t36934 + t36935 + t36936 + t36937 - 0.4419852458519115466e-7_f64 * t34433 - t36939 + t36940 - t36941;
    let t38578 = -t36956 + t36957 - t36958 + t36959 + t36960 - t36961 - t36962 - t36963 + 0.4419852458519115466e-7_f64 * t34505 - t36965 - 0.19666550313313802086e-6_f64 * t34510;
    (t38571, t38578)
}
