//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1027/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1027<F: Float>(t3190: F, t910: F, t551: F, t552: F, t2892: F, t938: F, t1569: F, t9955: F, t1568: F, t529: F, t6363: F, t6375: F, t9948: F, t538: F, t9956: F, t1577: F, t2651: F, t3077: F, t3081: F, t535: F, t574: F, t6218: F, t6362: F, t6449: F, t7490: F, t8198: F, t9178: F, t9180: F, t9219: F, t9221: F, t9223: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10125 = t3190 * t910;
    let t10127 = t551 * t552 * t10125;
    let t10130 = t2892 * t938;
    let t10132 = t551 * t552 * t10130;
    let t10135 = t9955 * t1569;
    let t10137 = t529 * t1568 * t10135;
    let t10140 = t9955 * t6363;
    let t10142 = t551 * t552 * t10140;
    let t10146 = t551 * t552 * t10135;
    let t10150 = t529 * t6375 * t10140;
    let t10156 = t551 * t552 * t9948;
    let t10160 = t529 * t538 * t9956;
    let t10164 = t551 * t552 * t9956;
    let t10175 = -0.7801399566048841707e0 * t6218 * t10127 - 0.15602799132097683414e1 * t6449 * t10132 + 0.16463622957338778996e0 * t535 * t10137 - 0.2600466522016280569e0 * t6362 * t10142 + 0.26004665220162805689e0 * t1577 * t10146 - 0.16463622957338778997e0 * t535 * t10150 - 0.13002332610081402845e0 * t2651 * t3081 - 0.43341108700271342816e-1 * t574 * t10156 - 0.27439371595564631661e-1 * t535 * t10160 - 0.43341108700271342816e-1 * t574 * t10164 + 0.26004665220162805689e0 * t8198 * t3077 + 0.17563392970889009434e0 * t7490 - 0.69345773920434148506e0 * t9178 - 0.38415120233790484326e0 * t9180 + 0.34672886960217074253e0 * t9219 + 0.19207560116895242163e0 * t9221 + 0.19207560116895242163e0 * t9223;
    (t10125, t10127, t10130, t10132, t10135, t10137, t10140, t10142, t10146, t10150, t10156, t10160, t10164, t10175)
}
