//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1198/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1198<F: Float>(t23939: F, t1416: F, t2484: F, t4998: F, t963: F, t1531: F, t7034: F, t19737: F, t481: F, t97: F, t1419: F, t2452: F, t2463: F, t5018: F, t5147: F, t5148: F, t8070: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t23940 = 0.7089e1 * t23939;
    let t23950 = t1416 * t2484;
    let t23951 = 60.0 * t23950;
    let t23953 = t963 * t4998;
    let t23955 = t7034 * t1531;
    let t23956 = 0.73245789224026180216e-3 * t23955;
    let t23961 = 96.0 * t19737;
    let t23962 = t97 * t481;
    let t23981 = t1419 * t2484;
    let t23982 = 36.0 * t23981;
    let t23985 = t1419 * t2452;
    let t23986 = 36.0 * t23985;
    let t23991 = t2463 * t5018;
    let t24016 = t5147 * t5148 * t8070;
    (t23940, t23951, t23953, t23956, t23961, t23962, t23982, t23986, t23991, t24016)
}
