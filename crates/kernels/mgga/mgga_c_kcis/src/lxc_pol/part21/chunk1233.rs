//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1233/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1233<F: Float>(t7787: F, t92794: F, t1094: F, t283: F, t1130: F, t15573: F, t27089: F, t7788: F, t27055: F, t27070: F, t26707: F, t2822: F) -> (F, F, F, F, F, F) {
    let t92795 = t7787 * t92794;
    let t92807 = t1094 * t283;
    let t92808 = t92807 * t1130;
    let t92814 = t7788 * t15573 * t27089;
    let t92816 = t27070 * t27055;
    let t92818 = t2822 * t26707;
    (t92795, t92807, t92808, t92814, t92816, t92818)
}
