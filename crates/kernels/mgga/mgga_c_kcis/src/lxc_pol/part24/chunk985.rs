//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 985/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk985<F: Float>(t1291: F, t8108: F, t1281: F, t8104: F, t28012: F, t28014: F, t28016: F, t28018: F, t28020: F, t28022: F, t28025: F, t28027: F, t28030: F, t28032: F, t28034: F, t28036: F, t28038: F) -> (F, F, F) {
    let t28260 = t8108 * t1291;
    let t28265 = t8104 * t1281;
    let t28280 = -0.25e0 * t28012 + 0.9375e-1 * t28014 - 0.20234375e-1 * t28016 + 0.625e-1 * t28018 - 0.10791666666666666667e0 * t28020 + 0.14388888888888888889e0 * t28022 - 0.89930555555555555557e-2 * t28025 + 0.20234375e-1 * t28027 - 0.4046875e-1 * t28030 - 0.20833333333333333333e-1 * t28032 + 0.26979166666666666667e-1 * t28034 - 0.625e-1 * t28036 - 0.26979166666666666667e-1 * t28038;
    (t28260, t28265, t28280)
}
