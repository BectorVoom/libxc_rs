//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1260/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1260<F: Float>(t1827: F, t3142: F, t1842: F, t159: F, t585: F, t617: F, t9005: F, t1732: F, t8997: F, t1768: F, t28494: F, t595: F, t637: F, t9056: F, t170: F, t60: F) -> (F, F, F, F, F, F, F) {
    let t28848 = t3142 * t1827;
    let t28850 = t3142 * t1842;
    let t28882 = t159 * t9005 * t585 * t617;
    let t28885 = t8997 * t1732;
    let t28887 = t28494 * t1768;
    let t28890 = t595 * t9056 * t637;
    let t28910 = t60 * t9005 * t170;
    (t28848, t28850, t28882, t28885, t28887, t28890, t28910)
}
