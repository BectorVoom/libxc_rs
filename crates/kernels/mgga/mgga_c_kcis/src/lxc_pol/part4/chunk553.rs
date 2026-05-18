//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 553/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk553<F: Float>(t1022: F, t2850: F, t1021: F, t1020: F, t359: F, t982: F) -> (F, F, F, F) {
    let t2851 = t1022 * t2850;
    let t2852 = t1021 * t2851;
    let t2853 = t1020 * t2852;
    let t2855 = t982 * t359;
    (t2851, t2852, t2853, t2855)
}
