//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 960/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk960<F: Float>(t11986: F, t23947: F, t23949: F, t23951: F, t23969: F, t28768: F, t28776: F, t28780: F, t28783: F, t28790: F, t28794: F, t29759: F, t7648: F, t9163: F) -> F {
    let t30003 = F::new(0.34822083333333333333e-2) * t28768 + F::new(0.46429444444444444443e-2) * t23947 - F::new(0.12381185185185185185e-1) * t23949 - F::new(0.46429444444444444443e-2) * t23951 + F::new(0.27857666666666666666e-1) * t28776 + F::new(0.30952962962962962963e-2) * t28780 + F::new(0.51072388888888888887e-1) * t28783 + F::new(0.579e0) * t7648 * t9163 - F::new(0.43134342e-1) * t11986 * t29759 + F::new(0.69644166666666666665e-2) * t23969 + F::new(0.69644166666666666666e-2) * t28790 + F::new(0.18571777777777777778e-1) * t28794;
    t30003
}
