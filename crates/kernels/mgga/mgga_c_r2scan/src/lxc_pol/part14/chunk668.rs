//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 668/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk668<F: Float>(t5465: F, t612: F, t1783: F, t585: F, t159: F, t617: F, t1883: F, t621: F, t632: F, t1891: F, t5448: F, t653: F, t219: F, t518: F, t201: F, t673: F, t681: F) -> (F, F, F, F, F, F, F) {
    let t5467 = 0.26345324029629629628e-2 * t612 * t5465;
    let t5468 = t1783 * t585;
    let t5469 = t159 * t5468;
    let t5470 = t5469 * t617;
    let t5474 = 6.0 * t632 * t1883 * t621;
    let t5479 = 0.57895126195293126241e3 * t1891 * t653 * t5448;
    let t5486 = t518 * t219;
    let t5490 = t518 * t201;
    let t5503 = t673 * t681;
    (t5467, t5470, t5474, t5479, t5486, t5490, t5503)
}
