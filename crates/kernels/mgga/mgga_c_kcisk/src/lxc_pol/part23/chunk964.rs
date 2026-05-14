//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 964/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk964<F: Float>(t19576: F, t334: F, t2079: F, t3638: F, t3641: F, t1171: F, t5712: F, t1192: F, t3672: F, t5715: F, t12884: F, t2105: F, t12888: F, t3697: F, t1201: F, t13042: F, t19456: F, t19459: F, t19473: F, t19478: F, t19480: F, t19482: F, t19559: F, t2107: F, t3692: F, t3718: F, t3726: F, t45: F, t5765: F, t5790: F, t5795: F) -> (F, F, F, F, F, F) {
    let t19577 = t19576 * t334;
    let t19580 = t2079 * t3638;
    let t19582 = 2.0 * t19580 * t3641;
    let t19583 = t5712 * t1171;
    let t19585 = 2.0 * t19583 * t1192;
    let t19587 = 1.0 * t5715 * t3672;
    let t19588 = t12884 * t2105;
    let t19589 = t12888 * t3697;
    let t19590 = t19588 * t19589;
    let t19593 = 0.11696446794910408142e1 * t1201 * t19456 - 0.35089340384731224426e1 * t1201 * t19459 - 0.34631511798751726598e2 * t3692 * t5795 - 0.17315755899375863299e2 * t5765 * t3726 - 0.58482233974552040708e0 * t13042 * t2107 - 0.58482233974552040708e0 * t5765 * t3718 - 0.11696446794910408142e1 * t3692 * t5790 - 0.34631511798751726598e2 * t1201 * t19473 + t19478 + t19480 + t19482 + t19559 + 0.19751789702565206229e-1 * t45 * t19577 - t19582 + t19585 + t19587 - 0.1025389702100779493e4 * t1201 * t19590;
    (t19577, t19582, t19585, t19587, t19590, t19593)
}
