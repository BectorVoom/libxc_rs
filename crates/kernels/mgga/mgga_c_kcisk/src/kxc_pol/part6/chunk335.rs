//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 335/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk335<F: Float>(t695: F, t786: F, t785: F, t657: F, t791: F, t1795: F, t801: F, t798: F, t1055: F, t143: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2014 = t786 * t695;
    let t2019 = t785 * t785;
    let t2020 = F::new(1.0) / t2019;
    let t2021 = t657 * t2020;
    let t2029 = F::new(1.0) / t791;
    let t2033 = F::cast_from(0.11607361111111111111e-2_f64) * t1795;
    let t2040 = t801 * t801;
    let t2041 = F::new(1.0) / t2040;
    let t2042 = t798 * t2041;
    let t2059 = -t143 - t1055;
    (t2014, t2019, t2020, t2021, t2029, t2033, t2040, t2041, t2042, t2059)
}
