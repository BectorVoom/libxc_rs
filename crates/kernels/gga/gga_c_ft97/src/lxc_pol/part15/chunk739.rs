//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 739/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk739<F: Float>(t20868: F, t9133: F, t12752: F, t16986: F, t17060: F, t17091: F, t1901: F, t20744: F, t20750: F, t20755: F, t20760: F, t20765: F, t20769: F, t20853: F, t20859: F, t20862: F, t20865: F, t28: F, t89: F) -> (F, F) {
    let t20869 = t9133 * t20868;
    let t20872 = -F::new(2.0) / F::new(9.0) * t16986 - F::new(2.0) / F::new(3.0) * t1901 * t20744 + t17060 / F::new(3.0) + F::new(2.0) / F::new(9.0) * t1901 * t20750 + F::new(2.0) / F::new(9.0) * t1901 * t20755 + t1901 * t20760 / F::new(3.0) + t1901 * t20765 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t1901 * t20769 - F::new(2.0) / F::new(3.0) * t17091 + t89 * t28 * t20853 / F::new(3.0) + F::new(4.0) / F::new(9.0) * t12752 - F::new(2.0) / F::new(9.0) * t1901 * t20859 + F::new(2.0) / F::new(3.0) * t1901 * t20862 + F::new(2.0) / F::new(3.0) * t1901 * t20865 - F::new(2.0) / F::new(3.0) * t1901 * t20869;
    (t20869, t20872)
}
