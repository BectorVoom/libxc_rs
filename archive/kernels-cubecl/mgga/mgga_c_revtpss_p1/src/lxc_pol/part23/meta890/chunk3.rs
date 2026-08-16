//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2834/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2834<F: Float>(t23281: F, t2652: F, t14785: F, t23148: F, t2477: F, t2745: F, t40607: F, t40611: F, t4433: F, t50607: F, t50608: F, t50611: F, t50615: F, t50619: F, t50634: F, t50681: F, t6017: F, t61833: F, t61839: F, t76572: F, t76583: F, t76587: F, t76591: F, t775: F, t828: F, t851: F) -> F {
    let t76593 = t2652 * t23281;
    let t76595 = -t50607 + F::cast_from(0.68026775414003982664e-1_f64) * t50608 + F::cast_from(0.24396650548625514667e-3_f64) * t50611 - F::cast_from(0.30492001685571196934e-4_f64) * t50615 - F::cast_from(0.15246000842785598467e-4_f64) * t50619 + F::cast_from(0.68026775414003982663e-1_f64) * t50634 - F::cast_from(0.12862205435420921092e-1_f64) * t2745 * t14785 * t6017 * t4433 - F::cast_from(0.15246000842785598467e-3_f64) * t61833 - F::cast_from(0.42874018118069736973e-4_f64) * t76572 + t40607 - t40611 - F::cast_from(0.6098400337114239387e-4_f64) * t61839 - F::cast_from(0.81312004494856525158e-3_f64) * t50681 + F::cast_from(0.42874018118069736972e-2_f64) * t851 * t2477 * t828 * t23148 * t775 - F::cast_from(0.42874018118069736973e-4_f64) * t76583 + F::cast_from(0.17149607247227894789e-3_f64) * t76587 + F::cast_from(0.7623000421392799234e-4_f64) * t76591 - F::cast_from(0.60023625365297631763e-1_f64) * t76593;
    t76595
}
