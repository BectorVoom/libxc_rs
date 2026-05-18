//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1367/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1367<F: Float>(t28219: F, t7784: F, t7772: F, t97024: F, t1856: F, t26996: F, t3611: F, t5329: F, t30066: F, t3532: F, t27042: F, t28113: F, t28118: F, t28125: F, t28153: F, t92795: F, t93023: F, t93028: F, t93082: F) -> (F, F, F) {
    let t97248 = F::new(0.23168402777777777778e-3) * t28219 * t7784;
    let t97250 = F::new(0.30918233506944444444e-4) * t7772 * t97024;
    let t97253 = t5329 * t26996 * t1856 * t3611;
    let t97258 = t5329 * t30066 * t1856 * t3532;
    let t97263 = F::new(0.46336805555555555556e-3) * t93023 * t28118 + F::new(0.30918233506944444444e-4) * t93028 * t28113 - F::new(0.30891203703703703704e-3) * t93023 * t28125 - F::new(0.12356481481481481482e-2) * t92795 * t28118 - F::new(0.82448622685185185185e-4) * t93082 * t28113 + F::new(0.8237654320987654321e-3) * t92795 * t28125 - t97248 - t97250 - F::new(0.46377350260416666667e-4) * t7772 * t97253 + F::new(0.92754700520833333334e-4) * t7772 * t97258 - F::new(0.24734586805555555556e-3) * t27042 * t28153;
    (t97253, t97258, t97263)
}
