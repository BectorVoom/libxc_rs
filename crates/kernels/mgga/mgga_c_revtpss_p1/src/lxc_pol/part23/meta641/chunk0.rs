//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2356/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2356<F: Float>(t2495: F, t9385: F, t2491: F, t744: F, t760: F, t2492: F, t2514: F, t9367: F, t9371: F, t200: F, t631: F, t202: F, t635: F) -> (F, F, F, F, F, F, F) {
    let t39815 = t2495 * t9385;
    let t39816 = t2491 * t744 * t39815;
    let t39818 = F::cast_from(0.69263436422725855036e2_f64) * t760 * t39816;
    let t39821 = t9367 * t2492 * t9371 * t2514;
    let t39823 = F::cast_from(0.61524113149298439947e4_f64) * t760 * t39821;
    let t39825 = F::new(1.0) / t200 / t631;
    let t39840 = F::new(1.0) / t202 / t635;
    (t39815, t39816, t39818, t39821, t39823, t39825, t39840)
}
