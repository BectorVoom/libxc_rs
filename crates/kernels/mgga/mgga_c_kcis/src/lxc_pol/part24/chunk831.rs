//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 831/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk831<F: Float>(t3255: F, t6586: F, t10271: F, t10414: F, t1102: F, t14115: F, t14321: F, t18536: F, t18539: F, t18543: F, t18548: F, t18552: F, t18556: F, t18559: F, t18563: F, t18567: F, t18571: F, t18575: F, t18579: F, t18582: F, t18584: F, t18586: F, t18588: F, t18590: F, t4587: F) -> F {
    let t18592 = t3255 * t6586;
    let t18594 = F::cast_from(0.32852148333333333333e-2_f64) * t14321 * t18536 - F::new(0.19711289e-2) * t10414 * t18539 + t10271 - F::cast_from(0.295669335e-2_f64) * t1102 * t18543 + F::cast_from(0.295669335e-2_f64) * t1102 * t18548 - F::new(0.59133867e-2) * t1102 * t18552 + F::new(0.39422578e-2) * t1102 * t18556 - F::new(0.19711289e-2) * t18559 - F::cast_from(0.2920190962962962963e-3_f64) * t14115 - F::new(0.19711289e-2) * t1102 * t18563 + F::cast_from(0.13140859333333333333e-2_f64) * t1102 * t18567 + F::cast_from(0.39422577999999999999e-2_f64) * t1102 * t18571 - F::cast_from(0.52563437333333333332e-2_f64) * t4587 * t18575 + F::new(0.98556445e-3) * t1102 * t18579 + F::cast_from(0.13140859333333333333e-2_f64) * t18582 - F::cast_from(0.87605728888888888887e-3_f64) * t18584 - F::cast_from(0.65704296666666666667e-3_f64) * t18586 + F::cast_from(0.73004774074074074073e-3_f64) * t18588 - F::cast_from(0.87605728888888888887e-3_f64) * t18590 + F::cast_from(0.43802864444444444445e-3_f64) * t18592;
    t18594
}
