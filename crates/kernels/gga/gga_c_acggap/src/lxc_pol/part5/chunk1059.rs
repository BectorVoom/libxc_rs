//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1059/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1059<F: Float>(t5118: F, t997: F, t3409: F, t4695: F, t4335: F, t3382: F, t4685: F, t1008: F, t4535: F, t5096: F, t3775: F, t5101: F) -> (F, F, F, F, F, F, F) {
    let t18651 = t997 * t5118;
    let t18653 = t3409 * t4695;
    let t18655 = t3409 * t4335;
    let t18657 = t3382 * t4685;
    let t18660 = t1008 * t4535;
    let t18672 = t1008 * t5096;
    let t18683 = t3775 * t5101;
    (t18651, t18653, t18655, t18657, t18660, t18672, t18683)
}
