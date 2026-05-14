//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1179/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1179<F: Float>(t34492: F, t34495: F, t34497: F, t34499: F, t34501: F, t34507: F, t34515: F, t34517: F, t34520: F, t34522: F, t34525: F, t34528: F, t34537: F, t34539: F, t34553: F, t34555: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36959 = 0.4637672555408563478e-4 * t34492;
    let t36960 = 0.15716995342493974597e-7 * t34495;
    let t36961 = 0.42206481990611010728e-7 * t34497;
    let t36962 = 0.14068827330203670243e-7 * t34499;
    let t36963 = 0.12817572129705434851e-5 * t34501;
    let t36965 = 0.12817572129705434851e-5 * t34507;
    let t36969 = 0.50603841145833333336e-5 * t34515;
    let t36970 = 0.25301920572916666668e-5 * t34517;
    let t36971 = 0.50603841145833333336e-5 * t34520;
    let t36972 = 0.25301920572916666668e-5 * t34522;
    let t36973 = 0.50603841145833333336e-5 * t34525;
    let t36974 = 0.48917046440972222224e-4 * t34528;
    let t36977 = 0.13111033542209201391e-7 * t34537;
    let t36978 = 0.14068827330203670243e-7 * t34539;
    let t36982 = 0.13506074236995523433e-5 * t34553;
    let t36983 = 0.13506074236995523433e-5 * t34555;
    (t36959, t36960, t36961, t36962, t36963, t36965, t36969, t36970, t36971, t36972, t36973, t36974, t36977, t36978, t36982, t36983)
}
