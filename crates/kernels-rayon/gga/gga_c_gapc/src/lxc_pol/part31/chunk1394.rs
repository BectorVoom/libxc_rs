//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1394/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1394(t34507: f64, t34515: f64, t34517: f64, t34520: f64, t34522: f64, t34525: f64, t34528: f64, t34537: f64, t34539: f64, t34553: f64, t34555: f64, t34557: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36965 = 0.12817572129705434851e-5_f64 * t34507;
    let t36969 = 0.50603841145833333336e-5_f64 * t34515;
    let t36970 = 0.25301920572916666668e-5_f64 * t34517;
    let t36971 = 0.50603841145833333336e-5_f64 * t34520;
    let t36972 = 0.25301920572916666668e-5_f64 * t34522;
    let t36973 = 0.50603841145833333336e-5_f64 * t34525;
    let t36974 = 0.48917046440972222224e-4_f64 * t34528;
    let t36977 = 0.13111033542209201391e-7_f64 * t34537;
    let t36978 = 0.14068827330203670243e-7_f64 * t34539;
    let t36982 = 0.13506074236995523433e-5_f64 * t34553;
    let t36983 = 0.13506074236995523433e-5_f64 * t34555;
    let t36984 = 0.67530371184977617164e-6_f64 * t34557;
    (t36965, t36969, t36970, t36971, t36972, t36973, t36974, t36977, t36978, t36982, t36983, t36984)
}
