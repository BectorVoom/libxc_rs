//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 984/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk984<F: Float>(t18574: F, t4579: F, t3269: F, t6334: F, t934: F, t3255: F, t6574: F, t6578: F, t1098: F, t6606: F, t6570: F, t6582: F, t6586: F, t10271: F, t10414: F, t1102: F, t14115: F, t14321: F, t18536: F, t18539: F, t18543: F, t18548: F, t18552: F, t18556: F, t18559: F, t18563: F, t18567: F, t18571: F, t4587: F) -> (F,) {
    let t18575 = t4579 * t18574;
    let t18579 = t3269 * t6334 * t934;
    let t18582 = t3255 * t6574;
    let t18584 = t3255 * t6578;
    let t18586 = t1098 * t6606;
    let t18588 = t3255 * t6570;
    let t18590 = t3255 * t6582;
    let t18592 = t3255 * t6586;
    let t18594 = 0.32852148333333333333e-2 * t14321 * t18536 - 0.19711289e-2 * t10414 * t18539 + t10271 - 0.295669335e-2 * t1102 * t18543 + 0.295669335e-2 * t1102 * t18548 - 0.59133867e-2 * t1102 * t18552 + 0.39422578e-2 * t1102 * t18556 - 0.19711289e-2 * t18559 - 0.2920190962962962963e-3 * t14115 - 0.19711289e-2 * t1102 * t18563 + 0.13140859333333333333e-2 * t1102 * t18567 + 0.39422577999999999999e-2 * t1102 * t18571 - 0.52563437333333333332e-2 * t4587 * t18575 + 0.98556445e-3 * t1102 * t18579 + 0.13140859333333333333e-2 * t18582 - 0.87605728888888888887e-3 * t18584 - 0.65704296666666666667e-3 * t18586 + 0.73004774074074074073e-3 * t18588 - 0.87605728888888888887e-3 * t18590 + 0.43802864444444444445e-3 * t18592;
    (t18594,)
}
