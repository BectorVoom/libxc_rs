//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 573/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk573<F: Float>(t4262: F, t5697: F, t1886: F, t997: F, t1008: F, t1881: F, t1901: F, t1896: F, t335: F, t367: F, t418: F, t4255: F, t4261: F, t4368: F, t4369: F, t4373: F, t4391: F, t4398: F, t4423: F, t4427: F, t5632: F, t5636: F, t5676: F, t5681: F, t5684: F, t5686: F, t5690: F, t5694: F) -> (F, F) {
    let t5698 = t4262 * t5697;
    let t5701 = t997 * t1886;
    let t5703 = t1008 * t1881;
    let t5705 = t997 * t1901;
    let t5707 = t997 * t1896;
    let t5709 = t4368 + 35.0 / 216.0 * t4369 + 0.42874018118069736972e-3 * t4373 - t335 * t5632 / 48.0 - t367 * t5636 / 96.0 - t367 * t5676 / 96.0 - 0.17149607247227894789e-2 * t418 * t5681 + 0.42874018118069736972e-2 * t5684 - 0.85748036236139473944e-3 * t5686 - 0.85748036236139473944e-3 * t418 * t5690 - t4255 * t5694 / 8.0 - t4261 * t5698 / 12.0 - t4391 + t4398 + t4423 + t4427 - 0.20007875121765877254e-1 * t5701 + 0.12862205435420921092e-2 * t5703 + 0.20007875121765877254e-2 * t5705 - 0.20007875121765877254e-2 * t5707;
    (t5698, t5709)
}
