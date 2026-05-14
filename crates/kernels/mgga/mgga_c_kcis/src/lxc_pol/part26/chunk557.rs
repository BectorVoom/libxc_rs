//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 557/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk557<F: Float>(t4260: F, t5919: F, t1552: F, t2051: F, t2055: F, t4281: F, t2039: F, t4249: F, t5627: F, t584: F, t583: F, t1546: F, t4261: F, t5880: F) -> (F, F, F, F, F, F, F, F) {
    let t5920 = t4260 * t5919;
    let t5922 = t2051 * t1552;
    let t5924 = t4281 * t2055;
    let t5926 = t4249 * t2039;
    let t5928 = t584 * t5627;
    let t5929 = t583 * t5928;
    let t5930 = t1546 * t5929;
    let t5932 = t4261 * t5880;
    (t5920, t5922, t5924, t5926, t5928, t5929, t5930, t5932)
}
