//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1356/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1356<F: Float>(t12252: F, t12261: F, t12263: F, t12265: F, t12542: F, t12543: F, t16731: F, t16852: F, t16855: F, t16858: F, t16860: F, t16863: F, t16865: F, t16887: F, t16890: F, t16895: F, t16898: F, t16901: F, t16904: F, t17126: F, t17131: F, t17148: F) -> F {
    let t17150 = F::new(0.18396666666666666667e-1) * t12252 + F::new(0.18396666666666666667e0) * t12261 - F::new(0.5519e-1) * t12263 - F::new(0.11038e0) * t12265 + F::new(0.19419375e1) * t16852 - F::new(0.412621875e-1) * t16855 - F::new(0.258925e1) * t16858 - F::new(0.1294625e1) * t16860 + F::new(0.16504875e0) * t16863 + F::new(0.82524375e-1) * t16865 + t17126 - F::new(0.20128333333333333333e0) * t16731 + F::new(0.16557e0) * t16887 + F::new(0.49671e0) * t16890 - t17131 - F::new(0.5519e-1) * t16895 - t12542 - t12543 - F::new(0.27595e-1) * t16898 - F::new(0.16557e0) * t16901 + F::new(0.33114e0) * t16904 + t17148;
    t17150
}
