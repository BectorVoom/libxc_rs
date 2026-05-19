//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 793/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk793<F: Float>(t9570: F, t2626: F, t676: F, t3869: F, t2434: F, t762: F, t1331: F, t3860: F, t1320: F, t3855: F, t186: F, t685: F, t793: F) -> (F, F, F, F, F, F, F, F) {
    let t9571 = F::new(96.0) * t9570;
    let t9572 = t676 * t2626;
    let t9574 = F::cast_from(0.32530743900905219526e-1_f64) * t3869 * t9572;
    let t9575 = t2434 * t762;
    let t9577 = F::cast_from(0.21687162600603479684e-1_f64) * t3869 * t9575;
    let t9578 = t3860 * t1331;
    let t9579 = F::new(36.0) * t9578;
    let t9580 = t1320 * t3855;
    let t9581 = F::new(12.0) * t9580;
    let t9586 = t685 * t793 * t186;
    (t9571, t9572, t9574, t9575, t9577, t9579, t9581, t9586)
}
