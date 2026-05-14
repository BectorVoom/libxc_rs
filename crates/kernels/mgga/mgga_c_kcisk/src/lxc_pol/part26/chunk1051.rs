//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1051/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1051<F: Float>(t15006: F, t15052: F, t15064: F, t1580: F, t27739: F, t27806: F, t27851: F, t27856: F, t27862: F, t27865: F, t27869: F, t27873: F, t27908: F, t27931: F, t4397: F, t541: F, t8324: F) -> (F,) {
    let t27934 = t27739 + t27806 + t27851 - 0.17990788716177317213e-1 * t4397 * t8324 - 0.17990788716177317213e-1 * t1580 * t27856 - 0.19989765240197019126e-2 * t15006 + t15052 + 0.15991812192157615301e-1 * t15064 + 0.29984647860295528689e-2 * t27862 + 0.17990788716177317213e-1 * t1580 * t27865 + 0.53972366148531951639e-1 * t1580 * t27869 - 0.71963154864709268852e-1 * t1580 * t27873 + 0.2698618307426597582e-1 * t27908 * t541 + t27931;
    (t27934,)
}
