//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1348/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1348<F: Float>(t1553: F, t7338: F, t1568: F, t7623: F, t1632: F, t549: F, t551: F, t7290: F, t2654: F, t6212: F, t20237: F, t6211: F, t2625: F, t20852: F, t20580: F, t20585: F, t20587: F, t20592: F, t20596: F, t20607: F, t20617: F, t20619: F, t20623: F, t20625: F) -> (F, F) {
    let t25466 = t7338 * t1553;
    let t25468 = t7623 * t1568 * t25466;
    let t25473 = t549 * t551 * t1632 * t7290;
    let t25480 = t6212 * t2654;
    let t25482 = t20237 * t6211 * t25480;
    let t25483 = 0.57131963037208741166e-1 * t25482;
    let t25486 = t6212 * t2625;
    let t25488 = t20852 * t6211 * t25486;
    let t25492 = -0.49390868872016336988e-1 * t25468 - 0.58218257753910989057e-2 * t20580 + 0.34672886960217074253e0 * t25473 - 0.41607464352260489103e1 * t20585 - 0.76280351312477563356e1 * t20587 + 0.38087975358139160777e-1 * t20592 - 0.1047928639570397803e0 * t20596 + 0.76280351312477563356e1 * t20607 + t25483 - 0.64025200389650807208e0 * t20617 + 0.64025200389650807209e-1 * t20619 + 0.22852785214883496467e0 * t25488 + 0.27744253502182016457e1 * t20623 - 0.20803732176130244552e1 * t20625;
    (t25466, t25492)
}
