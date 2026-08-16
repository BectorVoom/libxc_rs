//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1833/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1833(t3252: f64, t7363: f64, t7362: f64, t3248: f64, t1201: f64, t2152: f64, t24589: f64, t24760: f64, t24762: f64, t24765: f64, t24773: f64, t24778: f64, t24781: f64, t24785: f64, t24789: f64, t24792: f64, t3565: f64, t3604: f64, t470: f64, t7283: f64, t7373: f64, t7387: f64, t7389: f64) -> (f64, f64, f64, f64, f64) {
    let t24794 = t7363 * t3252;
    let t24795 = t7362 * t24794;
    let t24798 = t7363 * t3248;
    let t24799 = t7362 * t24798;
    let t24802 = -0.54831135561607547884e-2_f64 * t24760 - 0.82246703342411321825e-2_f64 * t7283 * t24762 - 0.16449340668482264365e-1_f64 * t7283 * t24765 + t3565 * t2152 + 2.0_f64 * t1201 * t7389 - t24773 + 2.0_f64 * t3604 * t7387 + 0.36554090374405031923e-2_f64 * t7283 * t24778 - 0.82246703342411321825e-2_f64 * t7283 * t24781 + 0.16449340668482264365e-1_f64 * t7373 * t24785 + 0.54831135561607547884e-2_f64 * t24589 * t24789 + t470 * t24792 - 0.27415567780803773942e-2_f64 * t7283 * t24795 - 0.54831135561607547884e-2_f64 * t7283 * t24799;
    (t24794, t24795, t24798, t24799, t24802)
}
