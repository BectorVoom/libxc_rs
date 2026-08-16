//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1086/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1086(t1250: f64, t9565: f64, t1014: f64, t7723: f64, t2179: f64, t3169: f64, t303: f64, t2865: f64, t355: f64, t359: f64, t342: f64, t2180: f64, t3245: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26823 = t9565 * t1250;
    let t26826 = t1014 * t7723;
    let t26828 = t3169 * t2179;
    let t26829 = t303 * t26828;
    let t26832 = t355 * t2865 * t359;
    let t26833 = t342 * t26832;
    let t26834 = t303 * t26833;
    let t26836 = t3245 * t2180;
    (t26823, t26826, t26828, t26829, t26832, t26833, t26834, t26836)
}
