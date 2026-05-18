//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1038/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1038<F: Float>(t31045: F, t3564: F, t13148: F, t2083: F, t7736: F, t13153: F, t2191: F, t30294: F, t5895: F, t1421: F, t19163: F, t19235: F, t26632: F, t26692: F, t31034: F, t31038: F, t31042: F) -> F {
    let t31046 = t3564 * t31045;
    let t31050 = t13148 * t7736 * t2083;
    let t31054 = t13153 * t7736 * t2191;
    let t31057 = t5895 * t30294;
    let t31060 = F::new(0.39422578e-2) * t26632 - F::new(0.98556445e-3) * t19163 - F::new(0.26281718666666666667e-2) * t26692 + F::new(0.65704296666666666665e-3) * t19235 - F::new(0.65704296666666666666e-2) * t1421 * t31034 + F::new(0.39422577999999999999e-2) * t1421 * t31038 - F::new(0.4435040025e-2) * t1421 * t31042 - F::new(0.4435040025e-2) * t1421 * t31046 + F::new(0.49278222499999999999e-2) * t1421 * t31050 - F::new(0.32852148333333333333e-2) * t1421 * t31054 + F::new(0.32852148333333333333e-2) * t1421 * t31057;
    t31060
}
