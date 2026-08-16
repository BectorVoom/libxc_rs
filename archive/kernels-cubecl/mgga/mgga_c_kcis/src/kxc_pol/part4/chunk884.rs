//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 884/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk884<F: Float>(t251: F, t584: F, t5676: F, t6010: F, t1466: F, t2033: F, t1535: F, t552: F, t5869: F, t577: F, t585: F, t1539: F, t2035: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6011 = t251 * t584;
    let t6012 = t6011 * t5676;
    let t6013 = t6010 * t6012;
    let t6015 = t2033 * t1466;
    let t6016 = t6015 * sigma2;
    let t6017 = t6016 * t1535;
    let t6019 = t5869 * t552;
    let t6020 = t6019 * t577;
    let t6021 = t6020 * t585;
    let t6023 = t2035 * t1539;
    (t6011, t6012, t6013, t6015, t6016, t6017, t6019, t6020, t6021, t6023)
}
