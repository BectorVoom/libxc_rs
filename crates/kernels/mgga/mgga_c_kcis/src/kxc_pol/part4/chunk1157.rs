//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1157/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1157<F: Float>(t10491: F, t5039: F, t3325: F, t5189: F, t1820: F, t3331: F, t10498: F, t1203: F, t3330: F, t3481: F, t13260: F, t5181: F) -> (F, F, F, F, F, F) {
    let t14674 = F::new(4.0) * t10491 * t5039;
    let t14676 = F::new(2.0) * t3325 * t5189;
    let t14677 = t1820 * t3331;
    let t14679 = F::new(6.0) * t10498 * t14677;
    let t14680 = t5189 * t1203;
    let t14682 = F::new(4.0) * t3330 * t14680;
    let t14683 = t1820 * t3481;
    let t14685 = F::new(2.0) * t3330 * t14683;
    let t14686 = t5181 * t13260;
    (t14674, t14676, t14679, t14682, t14685, t14686)
}
