//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1232/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1232<F: Float>(t410: F, t7705: F, t1838: F, t2747: F, t5352: F, t963: F, t5293: F, t1810: F, t5358: F, t1842: F, t1814: F, t5280: F, t5305: F, t5319: F, t1707: F, t2483: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t26589 = t410 * t7705;
    let t26590 = 24.0 * t26589;
    let t26595 = t2747 * t1838;
    let t26596 = 0.31168546390226634765e3 * t26595;
    let t26597 = t963 * t5352;
    let t26599 = t963 * t5293;
    let t26601 = t2747 * t1810;
    let t26602 = 0.10526802520742363173e2 * t26601;
    let t26603 = t963 * t5358;
    let t26607 = t2747 * t1842;
    let t26608 = 0.10389515463408878255e3 * t26607;
    let t26609 = t2747 * t1814;
    let t26610 = 0.51947577317044391277e2 * t26609;
    let t26611 = t963 * t5280;
    let t26613 = t963 * t5305;
    let t26615 = t963 * t5319;
    let t26622 = t2483 * t1707;
    (t26590, t26596, t26597, t26599, t26602, t26603, t26608, t26610, t26611, t26613, t26615, t26622)
}
