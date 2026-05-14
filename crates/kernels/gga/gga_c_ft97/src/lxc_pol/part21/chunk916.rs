//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 916/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk916<F: Float>(t27335: F, t3483: F, t27334: F, t23616: F, t23629: F, t23650: F, t27028: F, t27032: F, t27037: F, t27041: F, t27045: F, t27049: F, t27051: F, t27055: F, t27060: F, t27066: F, t27070: F, t27075: F, t27079: F, t27084: F, t27089: F, t27094: F, t27098: F, t27101: F, t27104: F, t27107: F, t27110: F) -> (F, F, F, F) {
    let t27336 = t27335 * t3483;
    let t27337 = t27334 * t27336;
    let t27351 = t27028 / 18.0 + t27032 / 9.0 + t27037 / 9.0 - 2.0 * t27041 + 2.0 / 9.0 * t27045 - t27049 / 6.0 - t27051 / 27.0 + t27055 / 3.0 - t23616 / 36.0 - t23629 / 9.0 - t27060 - t23650 / 54.0;
    let t27364 = -t27066 / 9.0 - t27070 / 9.0 + t27075 / 27.0 - t27079 / 36.0 - t27084 / 36.0 + t27089 / 12.0 + t27094 / 12.0 - 2.0 / 9.0 * t27098 - 2.0 / 9.0 * t27101 + 2.0 / 27.0 * t27104 - 2.0 / 9.0 * t27107 - t27110 / 9.0;
    (t27336, t27337, t27351, t27364)
}
