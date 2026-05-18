//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1197/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1197<F: Float>(t19333: F, t5393: F, t1248: F, t22346: F, t2843: F, t1091: F, t15312: F, t1901: F, t22208: F, t22405: F, t22410: F, t2857: F, t2862: F, t296: F, t319: F, t4246: F, t44523: F, t446: F, t4965: F, t5424: F, t840: F, t84331: F, t84357: F, t84390: F, t84404: F, t90313: F) -> (F, F, F) {
    let t90775 = t19333 * t5393;
    let t90785 = t2843 * t1248 * t22346;
    let t90799 = F::new(2.0) * t446 * t2862 * t319 * t90313 + F::new(8.0) / F::new(9.0) * t84331 - F::new(8.0) / F::new(3.0) * t1901 * t15312 * t22405 * t1091 - F::new(2.0) * t446 * t296 * t90775 + F::new(8.0) / F::new(3.0) * t1901 * t44523 * t22410 * t1091 - F::new(8.0) / F::new(9.0) * t84357 + F::new(8.0) / F::new(3.0) * t446 * t296 * t90785 + F::new(4.0) * t446 * t840 * t4246 * t22208 - F::new(4.0) / F::new(9.0) * t446 * t2857 * t5424 * t4965 + F::new(8.0) / F::new(9.0) * t84390 - F::new(8.0) / F::new(3.0) * t84404;
    (t90775, t90785, t90799)
}
