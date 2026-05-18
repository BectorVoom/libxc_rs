//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 837/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk837<F: Float>(t4757: F, t996: F, t1096: F, t1651: F, t1079: F, t2848: F, t3070: F, t4571: F, t4576: F, t4581: F, t4585: F) -> (F, F, F) {
    let t4758 = t996 * t4757;
    let t4763 = t1651 * t1096;
    let t4764 = t1079 * t4763;
    let t4772 = t3070 + F::new(0.4938888888888888889e-2) * t2848 + F::new(0.4938888888888888889e-2) * t4571 - F::new(0.9877777777777777778e-2) * t4576 + F::new(0.29633333333333333334e-1) * t4581 - F::new(0.14816666666666666667e-1) * t4585;
    (t4758, t4764, t4772)
}
