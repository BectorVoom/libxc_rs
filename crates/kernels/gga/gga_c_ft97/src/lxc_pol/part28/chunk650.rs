//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 650/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk650<F: Float>(t27066: F, t27070: F, t27075: F, t27079: F, t27084: F, t27089: F, t27094: F, t27098: F, t27101: F, t27104: F, t27107: F, t27110: F, t23890: F, t23914: F, t23920: F, t24034: F, t24041: F, t27116: F, t27121: F, t27126: F, t27130: F, t27133: F, t27135: F, t27139: F) -> (F, F) {
    let t27364 = -t27066 / 9.0 - t27070 / 9.0 + t27075 / 27.0 - t27079 / 36.0 - t27084 / 36.0 + t27089 / 12.0 + t27094 / 12.0 - 2.0 / 9.0 * t27098 - 2.0 / 9.0 * t27101 + 2.0 / 27.0 * t27104 - 2.0 / 9.0 * t27107 - t27110 / 9.0;
    let t27376 = -t27116 / 3.0 + t23890 / 18.0 - t24034 - t23914 / 27.0 + t23920 / 9.0 - t27121 / 9.0 + t27126 / 12.0 + t27130 / 3.0 + t27133 / 3.0 - t27135 / 36.0 - t24041 + t27139 / 18.0;
    (t27364, t27376)
}
