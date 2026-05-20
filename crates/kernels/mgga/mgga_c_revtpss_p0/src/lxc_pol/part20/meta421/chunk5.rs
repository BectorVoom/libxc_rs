//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1577/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1577<F: Float>(t43830: F, t43832: F, t43837: F, t43841: F, t43845: F, t43849: F, t43858: F, t43862: F, t43865: F, t43871: F, t43877: F, t43813: F) -> (F, F) {
    let t43880 = -F::new(40.0) / F::new(81.0) * t43858 - F::new(80.0) / F::new(81.0) * t43862 - F::new(8.0) / F::new(3.0) * t43830 - F::new(16.0) / F::new(27.0) * t43865 + F::new(8.0) / F::new(9.0) * t43832 + F::new(20.0) / F::new(9.0) * t43837 - F::new(2.0) / F::new(3.0) * t43871 - F::new(8.0) / F::new(9.0) * t43841 + F::new(12.0) * t43845 + F::new(2.0) * t43877 + F::new(8.0) / F::new(3.0) * t43849;
    let t43881 = F::new(280.0) / F::new(81.0) * t43813;
    (t43880, t43881)
}
