//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2891/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2891<F: Float>(t15475: F, t2874: F, t934: F, t15474: F, t2924: F, t2926: F, t11300: F, t11385: F, t4635: F, t2873: F, t4587: F, t2876: F) -> (F, F, F, F) {
    let t52495 = F::new(6.0) * t2874 * t15475 * t934;
    let t52499 = F::cast_from(0.48245938496077605201e2_f64) * t2924 * t15474 * t2926 * t934;
    let t52502 = F::cast_from(0.57895126195293126241e3_f64) * t11385 * t4635 * t11300;
    let t52505 = t4587 * t2873;
    let t52507 = F::new(6.0) * t52505 * t2876;
    (t52495, t52499, t52502, t52507)
}
