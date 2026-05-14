//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1254/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1254<F: Float>(t2860: F, t9073: F, t1150: F, t1162: F, t1170: F, t1179: F, t12042: F, t123: F, t26860: F, t26991: F, t27084: F, t27358: F, t27679: F, t27687: F, t27699: F, t27702: F, t27706: F, t27707: F, t27712: F, t27721: F, t27724: F, t27730: F, t27733: F, t27736: F, t27744: F, t3107: F, t3212: F, t3234: F, t429: F, t438: F, t4435: F, t449: F, t458: F, t894: F, t9062: F, t914: F) -> (F,) {
    let t27749 = t9073 * t2860;
    let t27753 = 0.58606582274942913081e3 * t27687 + 0.26372962023724310886e4 * t3212 * t458 * t27679 * t123 + 0.5848048239485271795e1 * t1170 * t894 * t449 * t26860 * t438 + 0.16829779668897917239e1 * t27699 + 0.1559479530529405812e2 * t27702 + 0.10508593825783314861e7 * t27706 * t458 * t27707 * t3107 - 0.75061384469880820436e5 * t27712 * t458 * t27707 * t438 + 0.30228422675018518373e0 * t1179 * t26991 + 0.35163949364965747848e4 * t27721 - 0.17581974682482873924e4 * t27724 + 0.30050434779516693818e0 * t1162 * t914 * t27084 + 0.18583473745796456084e3 * t27730 - 0.61944912485988186948e2 * t27733 + 0.12020173911806677527e0 * t27736 + 0.11360101276506094136e1 * t1150 * t914 * t429 * t26860 * t438 + 0.15146801702008125515e1 * t27744 + 0.69310201356862480534e2 * t3234 * t12042 * t27358 + 0.9291736872898228042e2 * t4435 * t9062 * t27749;
    (t27753,)
}
