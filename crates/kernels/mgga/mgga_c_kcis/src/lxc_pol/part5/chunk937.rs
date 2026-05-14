//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 937/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk937<F: Float>(t14576: F, t4793: F, t9429: F, t2861: F, t4815: F, t1017: F, t342: F, t86: F, t1130: F, t1767: F, t1697: F, t2835: F, t1141: F, t5034: F, t1778: F, t3329: F) -> (F, F, F, F, F, F, F, F) {
    let t14577 = 0.22109259259259259258e-2 * t14576;
    let t14607 = t9429 * t4793;
    let t14609 = t2861 * t4815;
    let t14627 = t86 * t1017 * t342;
    let t14628 = t1130 * t1767;
    let t14654 = t1697 * t2835;
    let t14665 = t5034 * t1141;
    let t14668 = t1778 * t3329;
    (t14577, t14607, t14609, t14627, t14628, t14654, t14665, t14668)
}
