//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 931/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk931<F: Float>(t8930: F, t9004: F, t2764: F, t898: F, t2770: F, t895: F, t897: F, t224: F, t2772: F, t906: F, t2789: F, t2150: F, t805: F) -> (F, F, F, F, F, F, F) {
    let t9005 = t8930 + t9004;
    let t9007 = t2764 * t898;
    let t9010 = t895 * t2770;
    let t9015 = t897 * t897;
    let t9016 = F::new(1.0) / t9015;
    let t9017 = t224 * t9016;
    let t9018 = t2772 * t906;
    let t9021 = t906 * t2789;
    let t9024 = t805 * t2150;
    (t9005, t9007, t9010, t9017, t9018, t9021, t9024)
}
