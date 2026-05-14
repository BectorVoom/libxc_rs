//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1034/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1034<F: Float>(t292: F, t1209: F, t14721: F, t14766: F, t22085: F, t2691: F, t2725: F, t285: F, t291: F, t4113: F, t43586: F, t5003: F, t5232: F, t5284: F, t70487: F, t70671: F, t800: F, t82848: F, t89941: F, t89994: F, t89999: F, t90003: F, t90008: F, t90015: F, t90054: F, t90088: F, t90168: F, t90204: F, t90234: F, t90264: F, t90300: F) -> (F,) {
    let t293 = 0.1e-59 < t292;
    let t90304 = piecewise3(t293, 2.0 * t800 * t291 * (t89941 + t89994) + 24.0 * t285 * t43586 * t89999 + 6.0 * t285 * t2725 * t90003 - 0.14498192132169191472e2 * t14766 * t90008 - 0.45910941751869106328e2 * t5232 * t5003 + 0.14498192132169191472e2 * t14721 * t90008 - 0.4127938044770952877e1 * t4113 * t90015 + 24.0 * t2691 * t70487 * t5284 - 24.0 * t70671 * t22085 - 0.65177969127962413846e0 * t82848 * t1209 + t90054 + t90088 + t90168 + t90204 + t90234 + t90264 + t90300, 0.0);
    (t90304,)
}
