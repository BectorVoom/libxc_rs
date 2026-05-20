//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2213/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2213<F: Float>(t4598: F, t6120: F, t4614: F, t11304: F, t15189: F, t18919: F, t18924: F, t18934: F, t23479: F, t23483: F, t23487: F, t23490: F, t23501: F, t23505: F) -> (F, F, F) {
    let t23521 = t4598 * t6120;
    let t23523 = t4614 * t6120;
    let t23535 = -t11304 - F::new(4.0) / F::new(9.0) * t15189 + F::new(2.0) / F::new(9.0) * t18919 - F::new(2.0) / F::new(3.0) * t18924 + t18934 / F::new(3.0) - F::new(10.0) / F::new(27.0) * t23479 + F::new(4.0) / F::new(3.0) * t23483 - F::new(2.0) / F::new(3.0) * t23501 - F::new(2.0) * t23487 + F::new(2.0) * t23505 - t23490 / F::new(3.0);
    (t23521, t23523, t23535)
}
