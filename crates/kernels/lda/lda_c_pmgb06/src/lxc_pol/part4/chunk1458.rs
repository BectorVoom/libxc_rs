//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1458/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1458<F: Float>(t8355: F, t8370: F, t8374: F, t11234: F, t18649: F, t1271: F, t2712: F, t955: F, t350: F, t365: F, t7018: F, t11230: F, t8358: F, t8376: F, t8382: F, t8386: F, t8388: F, t8390: F) -> (F, F, F, F, F) {
    let t18704 = F::new(3.031285185185185) * t8355;
    let t18706 = F::new(1.2991222222222223) * t8370;
    let t18707 = F::new(0.6495611111111111) * t8374;
    let t18716 = F::new(70.1526) * t11234 * t18649;
    let t18718 = t1271 * t2712 * t955;
    let t18721 = t365 * t7018 * t350;
    let t18723 = t18704 + F::new(28.0) / F::new(27.0) * t8358 + t18706 - t18707 + F::new(3.91744) * t8376 + F::new(2.0) / F::new(3.0) * t8382 + F::new(1.95872) * t8386 + F::new(3.91744) * t8388 - F::new(0.97936) * t8390 - F::new(117.5232) * t11230 * t18649 - t18716 + F::new(1.95872) * t18718 + F::new(1.46904) * t18721;
    (t18704, t18706, t18707, t18716, t18723)
}
