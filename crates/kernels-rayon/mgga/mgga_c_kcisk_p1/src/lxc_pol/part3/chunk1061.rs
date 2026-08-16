//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1061/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1061(t15781: f64, t15792: f64, t44: f64, t291: f64, t15174: f64, t15452: f64, t15457: f64, t15460: f64, t15463: f64, t15466: f64, t15471: f64, t15473: f64, t15763: f64, t15766: f64, t15767: f64, t15770: f64) -> f64 {
    let t15794 = (t15781 + t15792) * t44;
    let t15795 = t15794 * t291;
    let t15796 = -t15174 + t15452 - t15457 - t15460 - t15463 - t15466 - t15471 - t15473 + t15763 - t15766 + 3.0_f64 * t15767 - t15770 + t15795;
    t15796
}
