//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1103/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1103<F: Float>(t11848: F, t11850: F, t11853: F, t11896: F, t11899: F, t11904: F, t11908: F, t11913: F, t11916: F, t11919: F, t11922: F, t11925: F, t12093: F, t12109: F, t12115: F, t12133: F, t9182: F, t9183: F, t9192: F, t9194: F, t9196: F, t9214: F) -> F {
    let t12135 = -t9182 + F::cast_from(0.18396666666666666667e-1_f64) * t9183 + F::cast_from(0.18396666666666666667e0_f64) * t9192 - F::cast_from(0.5519e-1_f64) * t9194 - F::cast_from(0.11038e0_f64) * t9196 - t12093 + F::cast_from(0.82785e-1_f64) * t11848 + F::cast_from(0.91983333333333333334e-1_f64) * t11850 - t9214 + F::cast_from(0.19419375e1_f64) * t11853 + t12109 - F::cast_from(0.20128333333333333333e0_f64) * t11896 + F::cast_from(0.181155e1_f64) * t11899 + F::cast_from(0.12077e1_f64) * t11904 + F::cast_from(0.60385e0_f64) * t11908 - t12115 - F::cast_from(0.5519e-1_f64) * t11913 - F::cast_from(0.27595e-1_f64) * t11916 - F::cast_from(0.16557e0_f64) * t11919 + F::cast_from(0.33114e0_f64) * t11922 + F::cast_from(0.16557e0_f64) * t11925 + t12133;
    t12135
}
