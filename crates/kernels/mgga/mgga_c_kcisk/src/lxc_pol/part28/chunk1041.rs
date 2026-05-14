//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1041/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1041<F: Float>(t4811: F, t8889: F, t5074: F, t8867: F, t8871: F, t22315: F, t6675: F, t5184: F, t6674: F, t11218: F, t22320: F, t5192: F, t1801: F, t22592: F, t1800: F, t1799: F) -> (F, F, F, F, F, F, F, F) {
    let t23947 = t4811 * t8889;
    let t23949 = t5074 * t8867;
    let t23951 = t5074 * t8871;
    let t23953 = t6675 * t22315;
    let t23954 = t5184 * t23953;
    let t23955 = t6674 * t23954;
    let t23957 = t11218 * t22320;
    let t23958 = t5192 * t23957;
    let t23959 = t6674 * t23958;
    let t23961 = t1801 * t22592;
    let t23962 = t1800 * t23961;
    let t23963 = t1799 * t23962;
    (t23947, t23949, t23951, t23953, t23955, t23957, t23959, t23963)
}
