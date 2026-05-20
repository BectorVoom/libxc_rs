//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1081/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1081<F: Float>(t1518: F, t8686: F, t651: F, t1502: F, t7901: F, t8698: F, t1911: F, t33578: F, t33580: F, t33583: F, t34017: F, t34019: F, t34023: F, t34027: F, t8079: F, t8111: F, t8568: F, t8695: F) -> (F, F) {
    let t34028 = t8686 * t1518;
    let t34030 = F::new(2.0) * t651 * t34028;
    let t34031 = t1502 * t8686;
    let t34033 = F::new(3.0) * t8698 * t7901;
    let t34036 = t1911 * t8695 + F::new(3.0) * t8079 * t8568 - t8111 * t8568 - t33578 - t33580 - t33583 - t34017 - t34019 + t34023 - t34027 - t34030 - t34031 + t34033;
    (t34028, t34036)
}
