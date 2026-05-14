//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1209/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1209<F: Float>(t1139: F, t16926: F, t16710: F, t5095: F, t698: F, t1132: F, t16708: F, t16717: F, t16722: F, t16735: F, t16740: F, t16744: F, t16908: F, t12252: F, t12261: F, t12263: F, t12265: F, t12349: F, t12352: F, t16731: F, t16852: F, t16855: F, t16858: F, t16860: F, t16863: F, t16865: F, t16883: F, t16887: F, t16890: F, t16893: F, t16895: F, t16898: F, t16901: F, t16904: F) -> (F, F, F, F) {
    let t16927 = t1139 * t16926;
    let t16929 = 0.39862222222222222222e0 * t16710;
    let t16931 = t698 * t5095;
    let t16933 = t1132 * t16926;
    let t16940 = 0.36514074074074074075e-1 * t16908 + 0.3071625e0 * t16927 - t16929 + 0.13287407407407407408e0 * t16708 + 0.36514074074074074074e-1 * t16931 + 0.1898925e1 * t16933 - 0.11958666666666666667e1 * t16722 + 0.11958666666666666667e1 * t16740 + 0.59793333333333333334e0 * t16744 + 0.17938e1 * t16735 + 0.33218518518518518518e0 * t16717;
    let t16942 = 0.18257037037037037037e-1 * t12252 + 0.18257037037037037037e0 * t12261 - 0.54771111111111111111e-1 * t12263 - 0.10954222222222222222e0 * t12265 + 0.142419375e1 * t16852 - 0.76790625e-1 * t16855 - 0.1898925e1 * t16858 - 0.9494625e0 * t16860 + 0.3071625e0 * t16863 + 0.15358125e0 * t16865 + t16883 - 0.19931111111111111111e0 * t16731 + 0.16431333333333333333e0 * t16887 + 0.49293999999999999999e0 * t16890 - t16893 - 0.54771111111111111112e-1 * t16895 - t12349 - t12352 - 0.27385555555555555556e-1 * t16898 - 0.16431333333333333333e0 * t16901 + 0.32862666666666666666e0 * t16904 + t16940;
    (t16927, t16931, t16933, t16942)
}
