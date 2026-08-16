//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1230/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1230<F: Float>(t10110: F, t105258: F, t105267: F, t105423: F, t105428: F, t17052: F, t21054: F, t29080: F, t4268: F, t5636: F, t7087: F, t7830: F, t7841: F, t855: F, t86955: F, t98213: F, t98237: F) -> F {
    let t108361 = -F::cast_from(0.19739208802178717238e0_f64) * t105258 + F::cast_from(12.0_f64) * t4268 * t29080 - F::cast_from(0.9869604401089358619e-1_f64) * t98213 - F::cast_from(18.0_f64) * t855 * t10110 * t7841 * t5636 - F::cast_from(0.49348022005446793095e-1_f64) * t105267 - F::cast_from(0.14804406601634037928e0_f64) * t98237 + F::cast_from(6.0_f64) * t7087 * t21054 + F::cast_from(0.38381794893125283518e0_f64) * t86955 - F::cast_from(0.9869604401089358619e-1_f64) * t105423 + F::cast_from(6.0_f64) * t17052 * t7830 + F::cast_from(0.16449340668482264365e-1_f64) * t105428;
    t108361
}
