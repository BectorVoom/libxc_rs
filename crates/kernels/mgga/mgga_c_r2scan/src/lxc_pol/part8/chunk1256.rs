//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1256/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1256<F: Float>(t2035: F, t3034: F, t41: F, t595: F, t637: F, t9069: F, t9072: F, t9063: F, t9066: F, t1871: F, t3129: F, t584: F, t591: F, t9006: F, t1416: F, t3124: F) -> (F, F, F, F, F, F, F, F) {
    let t28564 = t41 * t3034 * t2035;
    let t28579 = t595 * t9069 * t637;
    let t28582 = t595 * t9072 * t637;
    let t28592 = t595 * t9063 * t637;
    let t28595 = t595 * t9066 * t637;
    let t28598 = t584 * t3129 * t1871;
    let t28601 = t584 * t9006 * t591;
    let t28615 = t1416 * t3124;
    (t28564, t28579, t28582, t28592, t28595, t28598, t28601, t28615)
}
