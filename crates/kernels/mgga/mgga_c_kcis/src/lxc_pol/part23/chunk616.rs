//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 616/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk616<F: Float>(t2039: F, t4249: F, t5627: F, t584: F, t583: F, t1546: F, t4261: F, t5880: F, t4260: F, t1551: F, t2061: F, t578: F) -> (F, F, F, F, F, F, F) {
    let t5926 = t4249 * t2039;
    let t5928 = t584 * t5627;
    let t5929 = t583 * t5928;
    let t5930 = t1546 * t5929;
    let t5932 = t4261 * t5880;
    let t5933 = t4260 * t5932;
    let t5935 = t2061 * t1551;
    let t5936 = t578 * t5935;
    (t5926, t5929, t5930, t5932, t5933, t5935, t5936)
}
