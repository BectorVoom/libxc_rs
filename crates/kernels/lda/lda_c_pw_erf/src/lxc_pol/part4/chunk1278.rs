//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1278/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1278<F: Float>(t11543: F, t11567: F, t11570: F, t11574: F, t11577: F, t11597: F, t142: F, t14485: F, t14500: F, t14876: F, t1550: F, t1554: F, t1729: F, t1733: F, t1880: F, t1881: F, t18901: F, t18906: F, t18936: F, t19013: F, t2645: F, t2806: F, t296: F, t4430: F, t4441: F, t454: F, t5616: F, t5735: F, t5783: F, t5925: F, t6016: F, t6025: F, t6130: F, t7075: F, t7214: F, t777: F) -> (F,) {
    let t19041 = 6.0 * t1733 * t18901 + 12.0 * t5735 * t4441 + 3.0 * t1733 * t18906 + 0.7926732703470741 * t11567 - 1.849570964143173 * t11597 + 24.0 * t6025 * t14500 + 12.0 * t14485 * t14876 + (t18936 + t19013) * t296 + 2.0 * t7214 * t1550 - t777 * t1554 * t142 * t5616 - 2.0 * t1881 * t6130 - 2.0 * t2645 * t2806 - 2.0 * t1881 * t7075 - 6.0 * t5783 * t11543 - 12.0 * t5783 * t11577 - 12.0 * t5783 * t11570 - 12.0 * t11574 * t4430 + 24.0 * t1729 * t1880 * t454 * t5925 - 6.0 * t11574 * t6016;
    (t19041,)
}
