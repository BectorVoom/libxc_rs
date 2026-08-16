//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1397/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1397(t34495: f64, t34497: f64, t34499: f64, t34501: f64, t34507: f64, t34505: f64, t34510: f64, t36956: f64, t36957: f64, t36958: f64, t36959: f64, t34515: f64) -> (f64, f64) {
    let t36960 = 0.15716995342493974597e-7_f64 * t34495;
    let t36961 = 0.42206481990611010728e-7_f64 * t34497;
    let t36962 = 0.14068827330203670243e-7_f64 * t34499;
    let t36963 = 0.12817572129705434851e-5_f64 * t34501;
    let t36965 = 0.12817572129705434851e-5_f64 * t34507;
    let t36967 = -t36956 + t36957 - t36958 + t36959 + t36960 - t36961 - t36962 - t36963 + 0.44198524585191154658e-7_f64 * t34505 - t36965 - 0.19666550313313802087e-6_f64 * t34510;
    let t36969 = 0.50603841145833333336e-5_f64 * t34515;
    (t36967, t36969)
}
