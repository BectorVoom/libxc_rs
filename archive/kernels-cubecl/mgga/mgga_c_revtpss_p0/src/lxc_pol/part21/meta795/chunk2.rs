//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2877/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2877<F: Float>(t41832: F, t4732: F, t981: F, t11524: F, t15525: F, t11299: F, t11300: F, t1610: F, t11112: F, t15101: F, t11116: F, t15421: F) -> (F, F, F, F, F) {
    let t52201 = F::cast_from(0.17315859105681463759e2_f64) * t981 * t4732 * t41832;
    let t52204 = F::cast_from(0.51947577317044391277e2_f64) * t981 * t15525 * t11524;
    let t52207 = F::cast_from(24.0_f64) * t11299 * t1610 * t11300;
    let t52209 = F::cast_from(6.0_f64) * t15101 * t11112;
    let t52211 = F::cast_from(0.48245938496077605201e2_f64) * t15421 * t11116;
    (t52201, t52204, t52207, t52209, t52211)
}
