//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1043/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1043<F: Float>(t32118: F, t32123: F, t32124: F, t32126: F, t32131: F, t32182: F, t32856: F, t32858: F, t32862: F, t32864: F, t32867: F, t32869: F, t651: F, t7007: F, t7586: F) -> F {
    let t32873 = -F::new(2.0) * t32869 * t651 - F::new(2.0) * t7007 * t7586 - t32118 - t32123 - t32124 + F::new(3.0) * t32126 + t32131 + t32182 - F::new(2.0) * t32856 - F::new(2.0) * t32858 - F::new(2.0) * t32862 - F::new(2.0) * t32864 - F::new(2.0) * t32867;
    t32873
}
