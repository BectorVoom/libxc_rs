//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 908/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk908<F: Float>(t3842: F, t6154: F, t729: F, t242: F, t27897: F, t1882: F, t6858: F, t6875: F, t24789: F, t3876: F, t1901: F, t193: F, t24841: F, t24843: F, t28417: F, t28422: F, t28426: F, t28430: F, t28434: F, t28438: F, t28441: F, t446: F, t89: F) -> (F, F, F, F) {
    let t28445 = t729 * t6154 * t3842;
    let t28448 = t242 * t27897;
    let t28451 = t1882 * t6858;
    let t28453 = t1882 * t6875;
    let t28455 = t24789 * t3876;
    let t28458 = t24841 / 9.0 + t24843 / 9.0 + t89 * t193 * t28417 / 3.0 + 2.0 / 3.0 * t446 * t28422 + t446 * t28426 / 3.0 + 2.0 / 3.0 * t446 * t28430 + 2.0 / 3.0 * t446 * t28434 + 2.0 / 3.0 * t446 * t28438 + 2.0 / 3.0 * t446 * t28441 + t446 * t28445 / 3.0 - t446 * t28448 / 3.0 + t28451 / 9.0 + t28453 / 9.0 + t1901 * t28455 / 9.0;
    (t28445, t28448, t28455, t28458)
}
