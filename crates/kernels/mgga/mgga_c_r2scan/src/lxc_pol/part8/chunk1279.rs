//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1279/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1279<F: Float>(t2147: F, t2148: F, t29936: F, t2155: F, t26260: F, t29279: F, t29283: F, t8088: F, t10024: F, t494: F, t113: F, t28320: F, t6063: F, t6493: F, t8873: F, t1632: F, t551: F, t566: F, t8629: F) -> (F, F, F, F, F, F, F, F, F) {
    let t29938 = t2147 * t2148 * t29936;
    let t29941 = t2155 * t26260 * t29279;
    let t29944 = t2155 * t8088 * t29283;
    let t29946 = t10024 * t494;
    let t29948 = t2155 * t8088 * t29946;
    let t29951 = t28320 * t113;
    let t29953 = t2155 * t6063 * t29951;
    let t29960 = t6493 * t8873;
    let t29964 = t566 * t551 * t1632 * t8629;
    (t29938, t29941, t29944, t29946, t29948, t29951, t29953, t29960, t29964)
}
