//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 641/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk641<F: Float>(t14053: F, t2574: F, t265: F, t3821: F, t766: F, t729: F, t762: F, t1175: F, t2409: F, t724: F, t2413: F, t3897: F, t2599: F, t2405: F, t9803: F, t1934: F, t992: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14055 = t2574 * t265 * t14053;
    let t14058 = t3821 * t766;
    let t14060 = t729 * t762 * t14058;
    let t14064 = t724 * t1175 * t2409;
    let t14067 = t3897 * t2413;
    let t14068 = t2599 * t14067;
    let t14071 = t3897 * t2405;
    let t14072 = t9803 * t14071;
    let t14075 = t992 * t1934;
    (t14055, t14058, t14060, t14064, t14067, t14068, t14071, t14072, t14075)
}
