//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1243/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1243<F: Float>(t13625: F, t25082: F, t32113: F, t26088: F, t531: F, t2014: F, t7238: F, t25090: F, t7235: F, t25803: F, t25802: F, t7312: F) -> (F, F, F, F, F) {
    let t94355 = F::new(18.0) * t25082 * t32113 * t13625;
    let t94358 = t531 * t26088;
    let t94361 = F::new(9.0) * t2014 * t94358 * t7238;
    let t94369 = F::new(9.0) * t7235 * t25090;
    let t94371 = F::new(3.0) * t7235 * t25803;
    let t94374 = F::new(3.0) * t2014 * t7312 * t25802;
    (t94355, t94361, t94369, t94371, t94374)
}
