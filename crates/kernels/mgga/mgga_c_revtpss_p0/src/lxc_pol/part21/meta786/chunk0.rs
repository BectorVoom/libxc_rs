//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2833/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2833<F: Float>(t11354: F, t2881: F, t4606: F, t11358: F, t15220: F, t2897: F, t918: F, t2880: F, t51849: F, t51853: F, t51858: F, t51863: F, t51867: F, t51871: F, t51875: F) -> (F, F, F, F, F) {
    let t51878 = t11354 * t4606 * t2881;
    let t51881 = t11358 * t4606 * t2881;
    let t51884 = t2897 * t15220 * t918;
    let t51887 = t2880 * t15220 * t918;
    let t51889 = F::new(0.72462e1) * t51849 - F::cast_from(0.20128333333333333333e0_f64) * t51853 - F::cast_from(0.89459259259259259259e0_f64) * t51858 + F::new(0.181155e1) * t51863 + F::new(0.181155e1) * t51867 + F::new(0.60385e0) * t51871 - F::new(0.72462e1) * t51875 + F::new(0.58258125e1) * t51878 - F::cast_from(0.1237865625e0_f64) * t51881 + F::cast_from(0.247573125e0_f64) * t51884 - F::new(0.3883875e1) * t51887;
    (t51878, t51881, t51884, t51887, t51889)
}
