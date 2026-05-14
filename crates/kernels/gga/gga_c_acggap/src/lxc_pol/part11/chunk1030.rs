//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1030/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1030<F: Float>(t34569: F, t8465: F, t5281: F, t7561: F, t35959: F, t35962: F, t35964: F, t35965: F, t35968: F, t35969: F, t35971: F, t35973: F, t35976: F, t35978: F, t35980: F, t35982: F, t35985: F, t35988: F, t35992: F, t35995: F) -> (F,) {
    let t35997 = t34569 * t8465;
    let t35998 = 0.94344276868812456204e-2 * t35997;
    let t35999 = t7561 * t5281;
    let t36001 = 0.85748036236139473944e-3 * t35959 + t35962 + t35964 + 0.85748036236139473944e-3 * t35965 - t35968 + 0.80031500487063509014e-2 * t35969 - 0.85748036236139473944e-3 * t35971 - 0.80031500487063509014e-2 * t35973 - t35976 + t35978 - t35980 + t35982 + 0.7145669686344956162e-3 * t35985 + t35988 + t35992 - 0.31448092289604152068e-2 * t35995 - t35998 + 0.34299214494455789578e-2 * t35999;
    (t36001,)
}
