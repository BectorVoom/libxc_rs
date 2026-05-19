//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1415/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1415<F: Float>(t34410: F, t34413: F, t34417: F, t34433: F, t36934: F, t36935: F, t36936: F, t36937: F, t36939: F, t36940: F, t36941: F, t34505: F, t34510: F, t36956: F, t36957: F, t36958: F, t36959: F, t36960: F, t36961: F, t36962: F, t36963: F, t36965: F) -> (F, F) {
    let t38571 = F::cast_from(0.6629778687778673199e-7_f64) * t34410 + F::cast_from(0.98332751566569010432e-8_f64) * t34413 - F::cast_from(0.89048050908546122982e-5_f64) * t34417 - t36934 + t36935 + t36936 + t36937 - F::cast_from(0.4419852458519115466e-7_f64) * t34433 - t36939 + t36940 - t36941;
    let t38578 = -t36956 + t36957 - t36958 + t36959 + t36960 - t36961 - t36962 - t36963 + F::cast_from(0.4419852458519115466e-7_f64) * t34505 - t36965 - F::cast_from(0.19666550313313802086e-6_f64) * t34510;
    (t38571, t38578)
}
