//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 967/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk967<F: Float>(t30310: F, t30314: F, t30319: F, t2304: F, t7610: F, t1988: F, t8561: F, t8566: F, t1181: F, t4521: F, t604: F, t7426: F) -> (F, F, F, F, F, F, F) {
    let t34211 = F::new(77.0) / F::new(864.0) * t30310;
    let t34212 = F::new(0.7640625e-2) * t30314;
    let t34214 = F::new(0.16006300097412701803e-1) * t30319;
    let t34215 = t7610 * t2304;
    let t34217 = t1988 * t8561;
    let t34221 = t1988 * t8566;
    let t34237 = t7426 * t1181 * t604 * t4521;
    (t34211, t34212, t34214, t34215, t34217, t34221, t34237)
}
