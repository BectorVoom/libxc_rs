//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1431/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1431<F: Float>(t2355: F, t8435: F, t27229: F, t7826: F, t1961: F, t33986: F, t33988: F, t33991: F, t33992: F, t33997: F, t34003: F, t34006: F, t34008: F, t34010: F, t34012: F, t34013: F, t34018: F, t34020: F, t34023: F, t3511: F, t35239: F, t5559: F, t841: F) -> (F, F) {
    let t35240 = t2355 * t8435;
    let t35242 = F::new(6.0) * t27229 * t7826;
    let t35243 = -F::new(6.0) * t1961 * t3511 * t5559 + F::new(2.0) * t1961 * t33992 - F::new(2.0) * t34013 * t841 - t33986 + t33988 + t33991 - t33997 + t34003 + t34006 - t34008 + t34010 - t34012 + t34018 - t34020 - t34023 + t35239 + t35240 - t35242;
    (t35240, t35243)
}
