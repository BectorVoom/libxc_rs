//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1135/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1135<F: Float>(t28071: F, t5036: F, t14668: F, t28002: F, t29081: F, t3325: F, t1008: F, t1704: F, t1662: F, t93426: F, t1009: F, t6539: F, t1003: F, t1709: F, t27772: F, t27778: F) -> (F, F, F, F, F, F, F) {
    let t100952 = 2.0 * t5036 * t28071;
    let t100954 = 4.0 * t14668 * t28002;
    let t100957 = t3325 * t29081;
    let t100970 = t1704 * t1008;
    let t100972 = t93426 * t1662 * t100970;
    let t100975 = t1009 * t6539;
    let t100983 = t27772 * t27778 * t1709 * t1003;
    (t100952, t100954, t100957, t100970, t100972, t100975, t100983)
}
