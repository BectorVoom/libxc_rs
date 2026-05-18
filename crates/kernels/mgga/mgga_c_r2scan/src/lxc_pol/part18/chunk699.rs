//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 699/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk699<F: Float>(t1982: F, t5456: F, t5460: F, t4: F, t518: F, t615: F, t612: F, t1883: F, t621: F, t632: F, t1891: F, t5448: F, t653: F) -> (F, F, F, F) {
    let t5461 = t1982 * t5456;
    let t5463 = F::new(0.30762056574649219974e4) * t5460 * t5461;
    let t5464 = t4 * t518;
    let t5465 = t615 * t5464;
    let t5467 = F::new(0.26345324029629629628e-2) * t612 * t5465;
    let t5474 = F::new(6.0) * t632 * t1883 * t621;
    let t5479 = F::new(0.57895126195293126241e3) * t1891 * t653 * t5448;
    (t5463, t5467, t5474, t5479)
}
