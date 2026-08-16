//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 606/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk606(t4262: f64, t5697: f64, t1886: f64, t997: f64, t1008: f64, t1881: f64, t1901: f64, t1896: f64, t335: f64, t367: f64, t418: f64, t4255: f64, t4261: f64, t4368: f64, t4369: f64, t4373: f64, t4391: f64, t4398: f64, t4423: f64, t4427: f64, t5632: f64, t5636: f64, t5676: f64, t5681: f64, t5684: f64, t5686: f64, t5690: f64, t5694: f64) -> (f64, f64) {
    let t5698 = t4262 * t5697;
    let t5701 = t997 * t1886;
    let t5703 = t1008 * t1881;
    let t5705 = t997 * t1901;
    let t5707 = t997 * t1896;
    let t5709 = t4368 + 35.0_f64 / 216.0_f64 * t4369 + 0.42874018118069736972e-3_f64 * t4373 - t335 * t5632 / 48.0_f64 - t367 * t5636 / 96.0_f64 - t367 * t5676 / 96.0_f64 - 0.17149607247227894789e-2_f64 * t418 * t5681 + 0.42874018118069736972e-2_f64 * t5684 - 0.85748036236139473944e-3_f64 * t5686 - 0.85748036236139473944e-3_f64 * t418 * t5690 - t4255 * t5694 / 8.0_f64 - t4261 * t5698 / 12.0_f64 - t4391 + t4398 + t4423 + t4427 - 0.20007875121765877254e-1_f64 * t5701 + 0.12862205435420921092e-2_f64 * t5703 + 0.20007875121765877254e-2_f64 * t5705 - 0.20007875121765877254e-2_f64 * t5707;
    (t5698, t5709)
}
