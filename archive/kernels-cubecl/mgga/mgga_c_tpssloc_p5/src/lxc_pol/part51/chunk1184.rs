//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1184/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1184<F: Float>(t1378: F, t31641: F, t31611: F, t6891: F, t6888: F, t6883: F, t8622: F, t22666: F, t8621: F, t1985: F, t225: F, t8618: F) -> (F, F, F, F, F, F, F) {
    let t31642 = t1378 * t31641;
    let t31645 = t31611 * t6891;
    let t31646 = t6888 * t31645;
    let t31648 = t6883 * t8622;
    let t31649 = F::cast_from(0.19190897446562641759e-1_f64) * t31648;
    let t31650 = t22666 * t8621;
    let t31651 = t1985 * t31650;
    let t31653 = t8618 * t225;
    (t31642, t31645, t31646, t31649, t31650, t31651, t31653)
}
