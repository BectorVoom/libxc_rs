//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1189/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1189<F: Float>(t1891: F, t7614: F, t1998: F, t6125: F, t30811: F, t6090: F, t31682: F, t31684: F, t35952: F, t35962: F, t35964: F, t35968: F, t35969: F, t35973: F, t35976: F, t35978: F, t35980: F, t35982: F, t37800: F, t37803: F, t37806: F) -> F {
    let t40385 = t7614 * t1891;
    let t40387 = t1998 * t6125;
    let t40390 = t30811 * t6090;
    let t40394 = F::new(0.27953859812981468504e-2) * t31682 + F::new(0.80031500487063509015e-2) * t40385 - F::new(0.85748036236139473944e-3) * t40387 - F::new(0.31448092289604152068e-3) * t31684 - t37800 - t35952 + t37803 + t37806 + t35962 + t35964 - t35968 - F::new(0.68598428988911579156e-2) * t40390 + F::new(0.80031500487063509015e-2) * t35969 - F::new(0.80031500487063509015e-2) * t35973 - t35976 + t35978 - t35980 + t35982;
    t40394
}
