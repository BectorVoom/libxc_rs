//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 990/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk990<F: Float>(t1843: F, t1911: F, t33578: F, t33580: F, t33583: F, t34017: F, t34019: F, t34023: F, t34027: F, t34030: F, t34031: F, t34776: F, t34788: F, t508: F, t569: F, t8886: F, t8897: F) -> (F,) {
    let t34790 = -t1843 * t8886 + t1911 * t8897 - t34776 * t508 + t34788 * t569 - t33578 - t33580 - t33583 - t34017 - t34019 + t34023 - t34027 - t34030 - t34031;
    (t34790,)
}
