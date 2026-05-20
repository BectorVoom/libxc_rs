//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1195/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1195<F: Float>(t12295: F, t12351: F, t12297: F, t12299: F, t12301: F, t12303: F, t12307: F, t12310: F, t12314: F, t12317: F, t12320: F, t12344: F, t12347: F, t12354: F) -> F {
    let t12542 = F::cast_from(0.93932222222222222223e0_f64) * t12295;
    let t12543 = F::cast_from(0.36793333333333333333e0_f64) * t12351;
    let t12546 = F::cast_from(0.20128333333333333333e0_f64) * t12299 + F::cast_from(0.33547222222222222222e0_f64) * t12307 + F::cast_from(0.40256666666666666668e0_f64) * t12297 - F::cast_from(0.60385000000000000001e0_f64) * t12301 - F::cast_from(0.30192500000000000001e0_f64) * t12303 - F::new(0.12077e1) * t12310 + F::new(0.181155e1) * t12314 + F::new(0.301925e0) * t12320 - F::new(0.3883875e1) * t12344 + F::cast_from(0.247573125e0_f64) * t12347 - t12542 - t12543 + F::new(0.181155e1) * t12317 + F::new(0.16504875e0) * t12354;
    t12546
}
