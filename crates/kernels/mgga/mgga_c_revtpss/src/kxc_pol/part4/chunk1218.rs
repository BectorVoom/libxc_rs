//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1218/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1218<F: Float>(t16892: F, t16708: F, t16710: F, t16717: F, t16722: F, t16735: F, t16740: F, t16744: F, t16908: F, t16927: F, t16931: F, t16933: F, t12252: F, t12261: F, t12263: F, t12265: F, t12542: F, t12543: F, t16731: F, t16852: F, t16855: F, t16858: F, t16860: F, t16863: F, t16865: F, t16887: F, t16890: F, t16895: F, t16898: F, t16901: F, t16904: F, t17126: F) -> (F,) {
    let t17131 = 0.22076e0 * t16892;
    let t17140 = 0.13418888888888888889e0 * t16708;
    let t17148 = 0.36793333333333333333e-1 * t16908 + 0.16504875e0 * t16927 - 0.40256666666666666667e0 * t16710 + t17140 + 0.36793333333333333334e-1 * t16931 + 0.258925e1 * t16933 - 0.12077e1 * t16722 + 0.12077e1 * t16740 + 0.60385e0 * t16744 + 0.181155e1 * t16735 + 0.33547222222222222222e0 * t16717;
    let t17150 = 0.18396666666666666667e-1 * t12252 + 0.18396666666666666667e0 * t12261 - 0.5519e-1 * t12263 - 0.11038e0 * t12265 + 0.19419375e1 * t16852 - 0.412621875e-1 * t16855 - 0.258925e1 * t16858 - 0.1294625e1 * t16860 + 0.16504875e0 * t16863 + 0.82524375e-1 * t16865 + t17126 - 0.20128333333333333333e0 * t16731 + 0.16557e0 * t16887 + 0.49671e0 * t16890 - t17131 - 0.5519e-1 * t16895 - t12542 - t12543 - 0.27595e-1 * t16898 - 0.16557e0 * t16901 + 0.33114e0 * t16904 + t17148;
    (t17150,)
}
