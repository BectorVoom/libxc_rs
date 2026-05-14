//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1096/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1096<F: Float>(t772: F, t25093: F, t79: F, t781: F, t24827: F, t2021: F, t1586: F, t2005: F, t2013: F, t2025: F, t25054: F, t25058: F, t2634: F, t2644: F, t7581: F, t7586: F, t7629: F, t7640: F, t782: F, t788: F, t9184: F, t9189: F, t9208: F, t9228: F) -> (F, F) {
    let t783 = 0.0 < t772;
    let t25094 = t79 * t25093;
    let t25095 = t25094 * t781;
    let t25101 = piecewise3(t783, t24827, -t24827);
    let t25102 = t2021 * t25101;
    let t25103 = t1586 * t25102;
    let t25116 = -0.89953943580886586067e-2 * t2013 * t25054 - 0.17990788716177317213e-1 * t2013 * t25058 - 0.17990788716177317213e-1 * t7581 * t7629 + 0.2698618307426597582e-1 * t25095 * t788 - 0.2698618307426597582e-1 * t2005 * t9228 - 0.2698618307426597582e-1 * t782 * t25103 - 0.2698618307426597582e-1 * t9208 * t2025 + 0.14392630972941853771e0 * t9189 * t2025 - 0.26386490117060065246e0 * t9184 * t2025 + 0.14392630972941853771e0 * t7586 * t2644 + 0.14392630972941853771e0 * t2634 * t7640;
    (t25101, t25116)
}
