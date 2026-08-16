//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1021/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1021(t14628: f64, t14645: f64, t14659: f64, t14672: f64, t14688: f64, t14701: f64, t14715: f64, t15079: f64, t1610: f64, t4528: f64, t1607: f64, t4534: f64) -> (f64, f64, f64) {
    let t15082 = t14628 + t14645 + t14659 + t14672 + t14688 + t14701 + t14715 + t15079;
    let t15084 = t4528 * t1610;
    let t15087 = t1607 * t4534;
    (t15082, t15084, t15087)
}
