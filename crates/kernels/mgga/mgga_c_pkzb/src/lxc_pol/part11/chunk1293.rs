//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1293/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1293<F: Float>(t18427: F, t18440: F, t18443: F, t18445: F, t27262: F, t27292: F, t27295: F, t31067: F, t31088: F, t31204: F, t31206: F, t31208: F, t31210: F, t31213: F, t31216: F, t31218: F, t31220: F, t31222: F, t31225: F) -> F {
    let t31493 = t18440 - F::cast_from(0.93932222222222222223e0_f64) * t18427 + t18443 - F::new(0.905775e0) * t27262 + F::new(0.82785e0) * t27292 + F::new(0.12077e1) * t27295 - F::cast_from(0.485484375e1_f64) * t31204 + F::new(0.58258125e1) * t31206 - F::new(0.3883875e1) * t31208 - F::new(0.3883875e1) * t31210 - F::new(0.1294625e1) * t31213 + F::cast_from(0.6189328125e-1_f64) * t31216 - F::cast_from(0.1237865625e0_f64) * t31218 + F::cast_from(0.247573125e0_f64) * t31220 + F::cast_from(0.247573125e0_f64) * t31222 + F::new(0.82524375e-1) * t31225 - F::new(0.301925e0) * t31067 + F::new(0.905775e0) * t31088 - F::cast_from(0.73586666666666666666e0_f64) * t18445;
    t31493
}
