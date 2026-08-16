//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 860/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk860(t12952: f64, t3661: f64, t26: f64, t1186: f64, t12957: f64, t3665: f64, t827: f64, t303: f64, t1175: f64, t3559: f64, t1394: f64, t298: f64, t301: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12961 = t3661 * t12952;
    let t12962 = t26 * t12961;
    let t12964 = t1186 * t12957;
    let t12965 = t26 * t12964;
    let t12967 = t827 * t3665;
    let t12969 = 1.0_f64/pow_3_2(t303);
    let t12970 = t3559 * t1175;
    let t12971 = t12969 * t12970;
    let t12974 = t298 * t1394 * t301;
    (t12962, t12965, t12967, t12970, t12971, t12974)
}
