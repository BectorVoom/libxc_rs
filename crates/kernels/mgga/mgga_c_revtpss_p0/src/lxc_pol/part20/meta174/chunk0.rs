//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 913/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk913<F: Float>(t1331: F, t3857: F, t189: F, t9363: F, t512: F, t3850: F, t72: F, t757: F, t2619: F, t3825: F, t1333: F, t3863: F) -> (F, F, F, F, F, F, F, F) {
    let t9559 = t3857 * t1331;
    let t9560 = F::new(60.0) * t9559;
    let t9561 = t9363 * t189;
    let t9562 = t512 * t9561;
    let t9563 = t3850 * t72;
    let t9564 = t9563 * t757;
    let t9565 = F::cast_from(0.54934341918019635162e-3_f64) * t9564;
    let t9566 = t3825 * t2619;
    let t9567 = F::cast_from(0.73245789224026180216e-3_f64) * t9566;
    let t9569 = F::new(60.0) * t3857 * t1333;
    let t9570 = t3863 * t1331;
    (t9560, t9561, t9562, t9563, t9565, t9567, t9569, t9570)
}
