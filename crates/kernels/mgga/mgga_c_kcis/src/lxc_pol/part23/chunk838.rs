//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 838/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk838<F: Float>(t486: F, t16958: F, t613: F, t1377: F, t3977: F, t3754: F, t1380: F, t5654: F, t1369: F, t1444: F, t16349: F, t1378: F, t286: F, t25: F, t5733: F, t493: F, t3999: F, t5732: F) -> (F, F, F, F, F, F, F, F, F) {
    let t495 = 0.0 < t486;
    let t16959 = t613 * t16958;
    let t16962 = t3977 * t1377;
    let t16963 = t16962 * t3754;
    let t16964 = t5654 * t1380;
    let t16965 = t16963 * t16964;
    let t16968 = t1369 * t1377;
    let t16969 = t16968 * t1444;
    let t16970 = t16969 * t16964;
    let t16974 = piecewise3(t495, t16349, -t16349);
    let t16975 = t1378 * t16974;
    let t16976 = t286 * t16975;
    let t16979 = t25 * t5733;
    let t16981 = t493 * t16979 / 144.0;
    let t16984 = t3999 * t5732;
    (t16959, t16962, t16965, t16968, t16970, t16974, t16976, t16981, t16984)
}
