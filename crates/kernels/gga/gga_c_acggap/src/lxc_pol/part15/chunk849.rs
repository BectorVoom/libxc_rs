//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 849/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk849<F: Float>(t157: F, t1838: F, t2152: F, t633: F, t1937: F, t2147: F, t9980: F, t8306: F, t9508: F, t1915: F, t1938: F, t2146: F, t2222: F, t2395: F, t557: F, t639: F, t7931: F, t8330: F, t8339: F, t8349: F, t9003: F, t9381: F, t9391: F, t9397: F, t9399: F, t9407: F, t9409: F, t9433: F, t9437: F, t9517: F) -> (F, F, F, F, F, F) {
    let t10011 = t2152 * t633 * t1838 * t157;
    let t10017 = t633 * t1937;
    let t10018 = t2147 * t10017;
    let t10022 = t2152 * t9980 * t157;
    let t10025 = t8306 * t9508;
    let t10038 = F::new(0.17347256376410398924e1) * t9003 * t2395 - F::new(0.65854491829355115987e0) * t2222 * t1938 + F::new(0.4336814094102599731e0) * t2146 * t10011 + F::new(0.13170898365871023197e1) * t2222 * t1915 - F::new(0.13170898365871023197e1) * t9381 + F::new(0.8673628188205199462e0) * t2146 * t10018 + F::new(0.4336814094102599731e0) * t2146 * t10022 + t8330 - F::new(0.17347256376410398924e1) * t7931 * t10025 - t8339 + F::new(0.13170898365871023197e1) * t9397 - F::new(0.13170898365871023197e1) * t9399 - F::new(0.17347256376410398924e1) * t9407 + F::new(0.17347256376410398924e1) * t9409 - F::new(0.4336814094102599731e0) * t9517 * t639 - F::new(0.13170898365871023197e1) * t9391 * t557 + t8349 - F::new(0.17347256376410398924e1) * t9433 + F::new(0.17347256376410398924e1) * t9437;
    (t10011, t10017, t10018, t10022, t10025, t10038)
}
