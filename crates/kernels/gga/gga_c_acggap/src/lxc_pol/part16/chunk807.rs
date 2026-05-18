//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 807/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk807<F: Float>(t8791: F, t9033: F, t159: F, t619: F, t8993: F, t2341: F, t309: F, t2147: F, t2131: F, t1659: F, t2127: F, t2149: F, t2159: F, t2338: F, t2342: F, t557: F, t616: F, t7912: F, t7929: F, t7931: F, t7944: F, t7950: F, t7957: F, t8001: F, t8400: F, t9003: F, t9026: F, t9031: F) -> (F, F, F, F, F) {
    let t9034 = t9033 * t8791;
    let t9044 = t619 * t159 * t8993;
    let t9053 = t2341 * t309;
    let t9054 = t2147 * t9053;
    let t9055 = t2131 * t9054;
    let t9057 = -F::new(0.8673628188205199462e0) * t7931 * t9026 + F::new(0.8673628188205199462e0) * t9031 - t7929 - F::new(0.8673628188205199462e0) * t8400 * t9034 + F::new(0.8673628188205199462e0) * t9003 * t2149 + F::new(0.8673628188205199462e0) * t7912 * t2342 - F::new(0.8673628188205199462e0) * t7944 + t7950 + F::new(0.65854491829355115987e0) * t7957 - F::new(0.4336814094102599731e0) * t616 * t9044 - F::new(0.65854491829355115987e0) * t2127 * t1659 - F::new(0.4336814094102599731e0) * t2338 * t2159 - F::new(0.65854491829355115987e0) * t8001 * t557 + F::new(0.17347256376410398924e1) * t9055;
    (t9034, t9044, t9054, t9055, t9057)
}
