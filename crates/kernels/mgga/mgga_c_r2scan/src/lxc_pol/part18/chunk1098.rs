//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1098/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1098<F: Float>(t2833: F, t545: F, t146: F, t2206: F, t2832: F, t10810: F, t1592: F, t8156: F, t10743: F, t2699: F, t37890: F, t924: F) -> (F, F, F, F, F) {
    let t39739 = t545 * t2833;
    let t39745 = t146 * t2206 * t2832;
    let t39762 = t1592 * t10810 * t8156;
    let t39763 = F::new(0.69345773920434148506e0) * t39762;
    let t39770 = t10743 * t2699;
    let t39771 = F::new(0.25610080155860322884e0) * t39770;
    let t39772 = t37890 * t924;
    (t39739, t39745, t39763, t39771, t39772)
}
