//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 598/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk598<F: Float>(t241: F, t258: F, t28097: F, t1175: F, t2574: F, t6079: F, t3977: F, t6088: F, t729: F, t6852: F, t773: F, t265: F, t27836: F, t27841: F, t242: F, t27913: F) -> (F, F, F, F, F, F, F) {
    let t28417 = t241 * t28097 * t258;
    let t28422 = t2574 * t1175 * t6079;
    let t28426 = t729 * t3977 * t6088;
    let t28430 = t2574 * t773 * t6852;
    let t28434 = t2574 * t265 * t27836;
    let t28438 = t2574 * t265 * t27841;
    let t28441 = t242 * t27913;
    (t28417, t28422, t28426, t28430, t28434, t28438, t28441)
}
