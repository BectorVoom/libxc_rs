//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1277/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1277<F: Float>(t23947: F, t2456: F, t1416: F, t2484: F, t4998: F, t963: F, t1531: F, t7034: F, t2: F, t464: F, t7007: F, t19737: F, t481: F, t97: F, t7142: F, t2266: F, t2330: F, t6967: F) -> (F, F, F, F, F, F, F, F) {
    let t23949 = 18.0 * t23947 * t2456;
    let t23950 = t1416 * t2484;
    let t23951 = 60.0 * t23950;
    let t23953 = t963 * t4998;
    let t23954 = 0.51947577317044391277e2 * t23953;
    let t23955 = t7034 * t1531;
    let t23956 = 0.73245789224026180216e-3 * t23955;
    let t23958 = t7007 * t2 * t464;
    let t23959 = 0.54934341918019635162e-3 * t23958;
    let t23961 = 96.0 * t19737;
    let t23962 = t97 * t481;
    let t23964 = 18.0 * t23962 * t7142;
    let t23968 = 18.0 * t2266 * t6967 * t2330 * t481;
    (t23949, t23951, t23954, t23956, t23959, t23961, t23964, t23968)
}
