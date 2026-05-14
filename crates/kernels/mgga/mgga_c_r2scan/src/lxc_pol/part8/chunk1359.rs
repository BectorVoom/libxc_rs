//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1359/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1359<F: Float>(t2139: F, t2294: F, t9990: F, t1567: F, t9955: F, t1569: F, t2115: F, t1604: F, t10007: F, t22744: F, t2122: F, t2124: F, t22749: F, t24839: F, t24859: F, t24883: F, t28418: F, t29354: F, t29363: F, t31131: F, t3116: F, t32777: F, t7321: F, t7984: F, t8837: F, t9166: F, t9170: F, t9521: F, t9544: F, t9548: F) -> (F, F, F) {
    let t33228 = t2139 * t2294 * t9990;
    let t33244 = t1567 * t9955;
    let t33245 = t33244 * t1569;
    let t33246 = t2115 * t33245;
    let t33247 = t1604 * t33246;
    let t33254 = t22744 * t10007;
    let t33256 = t24839 - t24859 - 0.10401866088065122276e1 * t33228 - 0.78013995660488417067e0 * t31131 * t9544 + 0.78013995660488417068e0 * t9521 * t9548 + 0.26004665220162805689e0 * t24883 * t3116 + 0.16463622957338778996e0 * t2122 * t2124 * t8837 * t28418 + 0.26004665220162805689e0 * t7984 * t9170 + 0.26004665220162805689e0 * t7984 * t9166 - 0.16463622957338778996e-1 * t33247 + 0.19756347548806534796e1 * t22749 * t7321 * t32777 + 0.11524536070137145298e1 * t29354 - 0.34672886960217074253e0 * t29363 + 0.11524536070137145297e1 * t33254;
    (t33244, t33246, t33256)
}
