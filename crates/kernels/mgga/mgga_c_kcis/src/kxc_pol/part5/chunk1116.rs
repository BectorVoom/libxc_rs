//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1116/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1116<F: Float>(t6338: F, t934: F, t6349: F, t932: F, t6320: F, t9758: F, t4625: F, t4657: F, t2943: F, t18685: F, t18645: F, t18650: F, t18655: F, t18659: F, t18661: F, t18664: F, t18667: F, t18669: F, t18674: F, t18679: F, t18683: F) -> (F, F, F, F, F, F, F) {
    let t18817 = t6338 * t934;
    let t18824 = t932 * t6349;
    let t18827 = t9758 * t6320;
    let t18828 = t18827 * t934;
    let t18830 = t4657 * t4625;
    let t18832 = t2943 * t6338;
    let t18833 = t18832 * t934;
    let t18835 = t932 * t18685;
    let t18853 = F::new(0.91722222222222222223e-3) * t18645 - F::new(0.45861111111111111112e-2) * t18650 + F::new(0.1651e-1) * t18655 - F::new(0.11006666666666666667e-1) * t18659 - F::new(0.27516666666666666667e-2) * t18661 - F::new(0.24765e-1) * t18664 + F::new(0.3302e-1) * t18667 + F::new(0.13758333333333333333e-2) * t18669 - F::new(0.27516666666666666667e-2) * t18674 + F::new(0.8255e-2) * t18679 - F::new(0.41275e-2) * t18683;
    (t18817, t18824, t18828, t18830, t18833, t18835, t18853)
}
