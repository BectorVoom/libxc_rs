//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 750/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk750<F: Float>(t1696: F, t468: F, t1376: F, t2: F, t464: F, t4798: F, t4806: F, t4827: F, t4839: F, t4988: F, t4992: F, t4996: F, t5000: F, t5004: F, t5008: F, t1520: F, t1531: F) -> (F, F, F, F, F, F, F) {
    let t5009 = t1696 * t468;
    let t5010 = 0.17544670867903938621e1 * t5009;
    let t5011 = t1376 * t2;
    let t5012 = t5011 * t464;
    let t5013 = 0.54934341918019635162e-3 * t5012;
    let t5014 = -t4798 + t4806 - t4988 - t4992 + t4996 - t5000 - t5004 - t5008 - t5010 - t4827 + t4839 - t5013;
    let t5015 = t1520 * t1531;
    (t5009, t5010, t5011, t5012, t5013, t5014, t5015)
}
