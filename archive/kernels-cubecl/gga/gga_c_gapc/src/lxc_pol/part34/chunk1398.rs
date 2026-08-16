//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1398/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1398<F: Float>(t34495: F, t34497: F, t34499: F, t34501: F, t34507: F, t34505: F, t34510: F, t36956: F, t36957: F, t36958: F, t36959: F, t34515: F) -> (F, F) {
    let t36960 = F::cast_from(0.15716995342493974597e-7_f64) * t34495;
    let t36961 = F::cast_from(0.42206481990611010728e-7_f64) * t34497;
    let t36962 = F::cast_from(0.14068827330203670243e-7_f64) * t34499;
    let t36963 = F::cast_from(0.12817572129705434851e-5_f64) * t34501;
    let t36965 = F::cast_from(0.12817572129705434851e-5_f64) * t34507;
    let t36967 = -t36956 + t36957 - t36958 + t36959 + t36960 - t36961 - t36962 - t36963 + F::cast_from(0.44198524585191154658e-7_f64) * t34505 - t36965 - F::cast_from(0.19666550313313802087e-6_f64) * t34510;
    let t36969 = F::cast_from(0.50603841145833333336e-5_f64) * t34515;
    (t36967, t36969)
}
