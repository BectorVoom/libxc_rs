//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2825/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2825<F: Float>(t2: F, t2838: F, t580: F, t895: F, t15091: F, t22: F, t265: F, t4567: F, t588: F, t15234: F, t2986: F, t974: F, t981: F) -> (F, F, F, F, F, F) {
    let t51827 = F::cast_from(3.0_f64) * t2838 * t2 * t580;
    let t51829 = F::cast_from(3.0_f64) * t895 * t580;
    let t51831 = F::cast_from(9.0_f64) * t15091 * t22;
    let t51833 = F::cast_from(6.0_f64) * t265 * t22;
    let t51835 = F::cast_from(12.0_f64) * t4567 * t588;
    let t51840 = F::cast_from(0.35089341735807877242e1_f64) * t981 * t2986 * t15234 * t974;
    (t51827, t51829, t51831, t51833, t51835, t51840)
}
