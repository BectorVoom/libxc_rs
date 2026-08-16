//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 804/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk804(t2409: f64, t904: f64, t2923: f64, t231: f64, t2918: f64, t9571: f64, t4342: f64, t9592: f64, t9556: f64, t10305: f64, t10308: f64, t10316: f64, t2417: f64, t9558: f64, t9560: f64, t9562: f64, t9564: f64, t9574: f64, t9580: f64, t9585: f64, t9589: f64, t9594: f64, t9598: f64) -> (f64, f64, f64, f64) {
    let t10870 = t2409 * t904;
    let t10871 = t2923 * t10870;
    let t10875 = t231 * t2918 * t9571;
    let t10877 = t4342 * t9592;
    let t10883 = 0.44934037037037037036e0_f64 * t9556;
    let t10894 = 0.1760655e0_f64 * t10305 - 0.352131e0_f64 * t10308 * t2417 + 0.234754e0_f64 * t10316 - t10883 - 0.19257444444444444444e0_f64 * t9558 + 0.9628722222222222222e-1_f64 * t9560 - 0.28886166666666666666e0_f64 * t9562 + 0.14443083333333333333e0_f64 * t9564 - 0.1604787037037037037e0_f64 * t9574 + 0.57772333333333333332e0_f64 * t9580 - 0.28886166666666666666e0_f64 * t9585 - 0.86658499999999999998e0_f64 * t9589 + 0.86658499999999999998e0_f64 * t9594 - 0.14443083333333333333e0_f64 * t9598;
    (t10871, t10875, t10877, t10894)
}
