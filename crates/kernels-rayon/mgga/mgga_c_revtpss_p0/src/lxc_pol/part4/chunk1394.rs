//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1394/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1394(t5056: f64, t5405: f64, t3626: f64, t12803: f64, t471: f64, t1715: f64, t12810: f64, t3603: f64, t3362: f64, t2251: f64, t5351: f64, t12787: f64) -> (f64, f64, f64, f64, f64) {
    let t17668 = t5056 * t5405;
    let t17669 = t3626 * t17668;
    let t17672 = t12803 * t471;
    let t17673 = t1715 * t17672;
    let t17674 = t3626 * t17673;
    let t17677 = t12810 * t3603;
    let t17678 = t1715 * t17677;
    let t17679 = t3626 * t17678;
    let t17682 = t12810 * t471;
    let t17683 = t1715 * t17682;
    let t17684 = t3626 * t17683;
    let t17687 = t471 * t3362;
    let t17688 = t17687 * t2251;
    let t17689 = t5351 * t17688;
    let t17690 = t12787 * t17689;
    (t17669, t17674, t17679, t17684, t17690)
}
