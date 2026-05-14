//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 564/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk564<F: Float>(t6010: F, t6012: F, t1466: F, t2033: F, t1535: F, t552: F, t5869: F, t577: F, t585: F, t1539: F, t2035: F, t1543: F, t2062: F, t4254: F, t492: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6013 = t6010 * t6012;
    let t6015 = t2033 * t1466;
    let t6016 = t6015 * sigma2;
    let t6017 = t6016 * t1535;
    let t6019 = t5869 * t552;
    let t6020 = t6019 * t577;
    let t6021 = t6020 * t585;
    let t6023 = t2035 * t1539;
    let t6025 = t1543 * t2062;
    let t6027 = t4254 * t492;
    (t6013, t6015, t6016, t6017, t6019, t6020, t6021, t6023, t6025, t6027)
}
