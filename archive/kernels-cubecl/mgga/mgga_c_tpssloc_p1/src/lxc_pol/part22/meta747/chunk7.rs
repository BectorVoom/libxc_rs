//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2495/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2495<F: Float>(t1003: F, t1058: F, t1060: F, t11046: F, t11048: F, t14618: F, t14651: F, t18099: F, t18121: F, t18155: F, t21615: F, t21622: F, t21626: F, t3200: F, t4615: F, t4657: F, t4669: F, t4684: F, t4691: F, t50592: F, t5866: F, t5903: F, t5937: F, t5939: F, t5941: F, t70014: F) -> F {
    let t70970 = F::cast_from(3.0_f64) * t1058 * t1060 * t4657 * t5866 + t11046 * t11048 * t70014 - F::cast_from(3.0_f64) * t18099 * t21622 * t3200 - F::cast_from(3.0_f64) * t21626 * t3200 * t4684 + t1003 * t21615 + F::cast_from(6.0_f64) * t14618 * t18121 + F::cast_from(3.0_f64) * t14651 * t5937 + F::cast_from(3.0_f64) * t18155 * t4669 + F::cast_from(3.0_f64) * t4615 * t5941 + F::cast_from(3.0_f64) * t4691 * t5903 - F::cast_from(3.0_f64) * t50592 * t5939;
    t70970
}
