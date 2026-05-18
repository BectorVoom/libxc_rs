//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 971/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk971<F: Float>(t1: F, t1742: F, t1750: F, t1755: F, t1752: F, t1753: F, t279: F, t2824: F, t3117: F, t3120: F, t3124: F, t3132: F) -> (F, F, F, F, F, F) {
    let t11260 = t1742 * t1750 * t1 * t1755;
    let t11266 = F::new(16.521134411652657) * t1752 * t1753 * t2824 * t279;
    let t11272 = F::new(192.98189186581325) * t3117;
    let t11273 = F::new(24.0) * t3120;
    let t11274 = F::new(24.0) * t3124;
    let t11275 = F::new(2069.0005882282467) * t3132;
    (t11260, t11266, t11272, t11273, t11274, t11275)
}
