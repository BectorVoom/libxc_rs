//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 695/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk695<F: Float>(t32065: F, t32092: F, t32061: F, t32072: F, t32080: F, t32085: F, t32089: F, t32097: F, t32101: F, t32104: F, t32109: F, t32113: F, t32331: F, t32348: F, t32118: F, t32123: F, t32328: F, t32336: F, t32341: F, t32345: F, t32353: F, t32358: F, t32362: F) -> (F, F, F, F, F, F, F) {
    let t32435 = 2.0 / 3.0 * t32065;
    let t32440 = t32092 / 3.0;
    let t32445 = 3.0 / 2.0 * t32061 + t32435 + 2.0 / 3.0 * t32072 + 4.0 * t32080 - 2.0 * t32085 - t32089 / 2.0 - t32440 - t32097 / 3.0 - 3.0 * t32101 + 2.0 * t32104 + t32109 / 4.0;
    let t32446 = t32113 / 6.0;
    let t32449 = 2.0 / 3.0 * t32331;
    let t32453 = t32348 / 3.0;
    let t32456 = t32446 + t32118 / 6.0 + t32123 - t32328 / 2.0 - t32449 - 2.0 / 3.0 * t32336 - 6.0 * t32341 + 4.0 * t32345 + t32453 + t32353 / 3.0 + 2.0 * t32358 - t32362;
    (t32435, t32440, t32445, t32446, t32449, t32453, t32456)
}
