//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 955/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk955<F: Float>(t3409: F, t5209: F, t1456: F, t3228: F, t1462: F, t1451: F, t3237: F, t4728: F, t997: F, t5118: F, t4695: F, t4335: F, t3382: F, t4685: F, t1008: F, t4535: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t18622 = t3409 * t5209;
    let t18628 = t3228 * t1456;
    let t18633 = t3228 * t1462;
    let t18647 = t3237 * t1451;
    let t18649 = t997 * t4728;
    let t18651 = t997 * t5118;
    let t18653 = t3409 * t4695;
    let t18655 = t3409 * t4335;
    let t18657 = t3382 * t4685;
    let t18660 = t1008 * t4535;
    (t18622, t18628, t18633, t18647, t18649, t18651, t18653, t18655, t18657, t18660)
}
