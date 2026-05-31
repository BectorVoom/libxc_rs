//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 993/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk993<F: Float>(t4598: F, t6120: F, t4614: F, t11304: F, t15189: F, t18919: F, t18924: F, t18934: F, t23479: F, t23483: F, t23487: F, t23490: F, t23501: F, t23505: F) -> (F, F, F) {
    let t23521 = t4598 * t6120;
    let t23523 = t4614 * t6120;
    let t23535 = -t11304 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t15189 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t18919 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t18924 + t18934 / F::cast_from(3.0_f64) - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t23479 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t23483 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t23501 - F::cast_from(2.0_f64) * t23487 + F::cast_from(2.0_f64) * t23505 - t23490 / F::cast_from(3.0_f64);
    (t23521, t23523, t23535)
}
